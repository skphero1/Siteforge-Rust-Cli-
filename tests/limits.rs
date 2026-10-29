use siteforge::{
    crawl::{Config, audit},
    network::Network,
};
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use url::Url;

type Hits = Arc<Mutex<Vec<(String, Instant)>>>;
fn server(mode: &'static str) -> (Url, Hits) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let hits: Hits = Arc::default();
    let recorded = hits.clone();
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            let mut stream = connection.unwrap();
            let recorded = recorded.clone();
            std::thread::spawn(move || {
                let mut request = [0; 8192];
                let n = stream.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..n]);
                let path = request.split_whitespace().nth(1).unwrap_or("/");
                recorded
                    .lock()
                    .unwrap()
                    .push((path.to_owned(), Instant::now()));
                let (status, headers, body) = match (path, mode) {
                    ("/robots.txt", "deny") => ("200 OK", "", "User-agent: *\nDisallow: /\n"),
                    ("/robots.txt", "delay") => {
                        ("200 OK", "", "User-agent: *\nCrawl-delay: 0.04\n")
                    }
                    ("/robots.txt" | "/sitemap.xml", _) => ("404 Not Found", "", ""),
                    ("/", "rate") => ("429 Too Many Requests", "Retry-After: 60\r\n", ""),
                    ("/", "loop") => ("302 Found", "Location: /\r\n", ""),
                    (_, "slow") => {
                        std::thread::sleep(Duration::from_millis(150));
                        ("200 OK", "", "slow")
                    }
                    (_, "large") => ("200 OK", "Content-Length: 99999\r\n", ""),
                    _ => (
                        "200 OK",
                        "Content-Type: text/html\r\n",
                        "<title>Test</title><h1>Test</h1><a href='/a'>A</a><a href='/b'>B</a><a href='/c'>C</a>",
                    ),
                };
                let length = if mode == "large" && path != "/robots.txt" && path != "/sitemap.xml" {
                    String::new()
                } else {
                    format!("Content-Length: {}\r\n", body.len())
                };
                let _ = stream.write_all(
                    format!(
                        "HTTP/1.1 {status}\r\n{headers}{length}Connection: close\r\n\r\n{body}"
                    )
                    .as_bytes(),
                );
            });
        }
    });
    (origin, hits)
}
fn config() -> Config {
    Config {
        max_pages: 2,
        concurrency: 2,
        delay: Duration::ZERO,
        network: Network {
            allow_private: true,
            timeout: Duration::from_secs(2),
            max_bytes: 2048,
        },
    }
}

#[tokio::test]
async fn page_limit_is_enforced() {
    let (url, hits) = server("normal");
    let report = audit(url, config()).await.unwrap();
    assert_eq!(report.pages.len(), 2);
    assert!(report.page_limit_reached);
    assert_eq!(hits.lock().unwrap().len(), 4); // robots + sitemap + two page jobs
}
#[tokio::test]
async fn rate_limit_stops_without_retries() {
    let (url, hits) = server("rate");
    let report = audit(url, config()).await.unwrap();
    assert!(report.stopped);
    assert_eq!(report.pages[0].status, Some(429));
    assert_eq!(hits.lock().unwrap().len(), 3);
}
#[tokio::test]
async fn robots_deny_prevents_sitemap_and_page_fetches() {
    let (url, hits) = server("deny");
    let report = audit(url, config()).await.unwrap();
    assert!(report.pages.is_empty());
    assert_eq!(report.skipped[0].reason, "robots_disallowed");
    assert_eq!(hits.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn redirects_cannot_loop_forever() {
    let (url, hits) = server("loop");
    let report = audit(url, config()).await.unwrap();
    assert_eq!(report.pages[0].error.as_deref(), Some("redirect loop"));
    assert_eq!(hits.lock().unwrap().len(), 3);
}
#[tokio::test]
async fn crawl_delay_applies_to_discovery_and_pages() {
    let (url, hits) = server("delay");
    audit(url, config()).await.unwrap();
    for pair in hits.lock().unwrap().windows(2) {
        assert!(pair[1].1.duration_since(pair[0].1) >= Duration::from_millis(30));
    }
}
#[tokio::test]
async fn network_enforces_body_and_time_limits() {
    let (url, _) = server("large");
    let network = config().network;
    assert!(
        network
            .get(&url)
            .await
            .unwrap_err()
            .to_string()
            .contains("body limit")
    );
    let (url, _) = server("slow");
    let network = Network {
        timeout: Duration::from_millis(25),
        ..network
    };
    assert!(network.get(&url).await.is_err());
}

#[tokio::test]
async fn configured_concurrency_is_a_hard_ceiling() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let active = Arc::new(AtomicUsize::new(0));
    let maximum = Arc::new(AtomicUsize::new(0));
    let observed = maximum.clone();
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            let mut stream = connection.unwrap();
            let active = active.clone();
            let maximum = maximum.clone();
            std::thread::spawn(move || {
                let mut bytes = [0; 8192];
                let n = stream.read(&mut bytes).unwrap();
                let request = String::from_utf8_lossy(&bytes[..n]);
                let path = request.split_whitespace().nth(1).unwrap();
                let body =
                    "<a href='/1'>1</a><a href='/2'>2</a><a href='/3'>3</a><a href='/4'>4</a>";
                let policy = path == "/robots.txt" || path == "/sitemap.xml";
                if !policy {
                    let count = active.fetch_add(1, Ordering::SeqCst) + 1;
                    maximum.fetch_max(count, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(50));
                    active.fetch_sub(1, Ordering::SeqCst);
                }
                let response = if policy {
                    "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        .to_owned()
                } else {
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                };
                let _ = stream.write_all(response.as_bytes());
            });
        }
    });
    let report = audit(
        url,
        Config {
            max_pages: 5,
            ..config()
        },
    )
    .await
    .unwrap();
    assert_eq!(report.pages.len(), 5);
    assert_eq!(observed.load(Ordering::SeqCst), 2);
}
