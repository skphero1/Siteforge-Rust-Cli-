use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_siteforge"))
}

#[test]
fn rejects_private_and_invalid_arguments() {
    for args in [
        vec!["audit", "http://127.0.0.1"],
        vec!["audit", "file:///etc/passwd"],
        vec!["audit", "https://example.com", "--concurrency", "0"],
    ] {
        assert!(!cli().args(args).output().unwrap().status.success());
    }
}

#[test]
fn audits_fixture_and_renders_reports() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            let mut stream = connection.unwrap();
            let mut buffer = [0; 8192];
            let n = stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            let path = request.split_whitespace().nth(1).unwrap_or("/");
            assert_ne!(path, "/private", "robots exclusion was fetched");
            let (status, headers, body) = match path {
                "/robots.txt" => ("200 OK", "", "User-agent: *\nDisallow: /private\n"),
                "/sitemap.xml" => ("200 OK", "", "<urlset></urlset>"),
                "/broken" => ("404 Not Found", "", "missing"),
                "/redirect" => ("302 Found", "Location: /broken\r\n", ""),
                _ => (
                    "200 OK",
                    "Content-Type: text/html\r\n",
                    include_str!("fixtures/page.html"),
                ),
            };
            let response = format!(
                "HTTP/1.1 {status}\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });
    for format in ["json", "csv", "html", "terminal"] {
        let output = cli()
            .args([
                "audit",
                &format!("http://{address}"),
                "--allow-private",
                "--delay-ms",
                "0",
                "--max-pages",
                "4",
                "--format",
                format,
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("broken"));
        if format == "json" {
            let report: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(report["pages"].as_array().unwrap().len(), 3);
            assert!(text.contains("missing_alt"));
            assert!(text.contains("wordpress"));
            assert!(text.contains("robots_disallowed"));
        }
    }
}

#[test]
fn redirect_to_metadata_is_never_followed() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            let mut stream = connection.unwrap();
            let mut buffer = [0; 8192];
            let n = stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            let path = request.split_whitespace().nth(1).unwrap_or("/");
            let response = if path == "/robots.txt" || path == "/sitemap.xml" {
                "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            } else {
                "HTTP/1.1 302 Found\r\nLocation: http://169.254.169.254/latest/meta-data\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            };
            let _ = stream.write_all(response.as_bytes());
        }
    });
    let output = cli()
        .args([
            "audit",
            &format!("http://{address}"),
            "--allow-private",
            "--delay-ms",
            "0",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["pages"][0]["error"],
        "redirect leaves allowed origin"
    );
}

#[test]
fn refuses_inaccessible_robots() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0; 8192];
        let _ = stream.read(&mut buffer);
        stream
            .write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
    });
    let output = cli()
        .args(["audit", &format!("http://{address}"), "--allow-private"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("robots.txt returned 403"));
}
