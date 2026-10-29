use crate::{
    html::{self, Seo},
    network::{Network, same_origin, validate_url},
    robots::Robots,
};
use anyhow::{Result, bail};
use serde::Serialize;
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct Config {
    pub max_pages: usize,
    pub concurrency: usize,
    pub delay: Duration,
    pub network: Network,
}
#[derive(Serialize)]
pub struct Page {
    pub url: Url,
    pub status: Option<u16>,
    pub redirects: Vec<Url>,
    pub error: Option<String>,
    pub seo: Option<Seo>,
}
#[derive(Serialize)]
pub struct Report {
    pub version: &'static str,
    pub origin: Url,
    pub robots_status: u16,
    pub sitemap_status: Option<u16>,
    pub sitemap_note: String,
    pub pages: Vec<Page>,
    pub skipped: Vec<Skipped>,
    pub stopped: bool,
    pub page_limit_reached: bool,
    pub frontier_truncated: bool,
}
#[derive(Serialize)]
pub struct Skipped {
    pub url: Url,
    pub reason: String,
}
struct Session {
    config: Config,
    origin: Url,
    robots: Robots,
    gate: Mutex<tokio::time::Instant>,
    stopped: Mutex<bool>,
}
impl Session {
    async fn get(&self, url: &Url) -> Result<crate::network::Response> {
        let mut next = self.gate.lock().await;
        tokio::time::sleep_until(*next).await;
        if *self.stopped.lock().await {
            bail!("origin stopped after access denial or rate limit");
        }
        *next = tokio::time::Instant::now() + self.config.delay;
        drop(next);
        let response = self.config.network.get(url).await?;
        if matches!(response.status.as_u16(), 401 | 403 | 429 | 503) {
            *self.stopped.lock().await = true;
        }
        Ok(response)
    }
    async fn page(&self, initial: Url) -> (Page, Vec<Url>) {
        let mut page = Page {
            url: initial.clone(),
            status: None,
            redirects: Vec::new(),
            error: None,
            seo: None,
        };
        let mut current = initial;
        let mut seen = HashSet::new();
        for _ in 0..=10 {
            if !seen.insert(current.clone()) {
                page.error = Some("redirect loop".into());
                break;
            }
            let path = path_query(&current);
            if !self.robots.allows(&path) {
                page.error = Some("redirect blocked by robots.txt".into());
                break;
            }
            let response = match self.get(&current).await {
                Ok(r) => r,
                Err(e) => {
                    page.error = Some(format!("{e:#}"));
                    break;
                }
            };
            page.status = Some(response.status.as_u16());
            if response.status.is_redirection() && response.status.as_u16() != 304 {
                let Some(location) = response.location else {
                    page.error = Some("redirect missing Location".into());
                    break;
                };
                let target = match current.join(&location) {
                    Ok(mut u) => {
                        u.set_fragment(None);
                        u
                    }
                    Err(_) => {
                        page.error = Some("invalid redirect URL".into());
                        break;
                    }
                };
                if validate_url(&target).is_err() || !same_origin(&target, &self.origin) {
                    page.error = Some("redirect leaves allowed origin".into());
                    break;
                }
                page.redirects.push(target.clone());
                current = target;
                if page.redirects.len() == 10 {
                    page.error = Some("redirect limit exceeded".into());
                    break;
                }
                continue;
            }
            if response.status.is_success() && response.html {
                let (seo, links) = html::analyze(&response.body, &current);
                page.seo = Some(seo);
                return (page, links);
            }
            break;
        }
        (page, Vec::new())
    }
}
fn path_query(url: &Url) -> String {
    match url.query() {
        Some(q) => format!("{}?{q}", url.path()),
        None => url.path().into(),
    }
}
pub async fn audit(mut origin: Url, mut config: Config) -> Result<Report> {
    validate_url(&origin)?;
    origin.set_fragment(None);
    if config.max_pages == 0 || config.concurrency == 0 {
        bail!("crawl limits must be positive");
    }
    let robots_url = origin.join("/robots.txt")?;
    let robots_response = config.network.get(&robots_url).await?;
    let robots_status = robots_response.status.as_u16();
    let robots = match robots_status {
        200 => Robots::parse(&robots_response.body),
        404 | 410 => Robots::default(),
        _ => {
            bail!("robots.txt returned {robots_status}; refusing to crawl without a usable policy")
        }
    };
    if robots.delay > 86400.0 {
        bail!("robots crawl-delay exceeds one day; refusing crawl");
    }
    config.delay = config.delay.max(Duration::from_secs_f64(robots.delay));
    let first_request = tokio::time::Instant::now() + config.delay;
    let session = Arc::new(Session {
        config,
        origin: origin.clone(),
        robots,
        gate: Mutex::new(first_request),
        stopped: Mutex::new(false),
    });
    let sitemap = origin.join("/sitemap.xml")?;
    let (sitemap_status, sitemap_note) = if session.robots.allows("/sitemap.xml") {
        match session.get(&sitemap).await {
            Ok(r) => {
                let note = if r.status.is_success()
                    && (r.body.contains("<urlset") || r.body.contains("<sitemapindex"))
                {
                    "sitemap XML root detected (not schema validated)"
                } else {
                    "missing, redirected, or unrecognized sitemap"
                };
                (Some(r.status.as_u16()), note.into())
            }
            Err(e) => (None, format!("sitemap fetch failed: {e}")),
        }
    } else {
        (None, "robots_disallowed".into())
    };
    let mut report = Report {
        version: env!("CARGO_PKG_VERSION"),
        origin: origin.clone(),
        robots_status,
        sitemap_status,
        sitemap_note,
        pages: Vec::new(),
        skipped: Vec::new(),
        stopped: false,
        page_limit_reached: false,
        frontier_truncated: false,
    };
    let mut queue = VecDeque::from([origin.clone()]);
    let mut known = HashSet::from([origin]);
    while !queue.is_empty()
        && report.pages.len() < session.config.max_pages
        && !*session.stopped.lock().await
    {
        let mut tasks = tokio::task::JoinSet::new();
        while tasks.len() < session.config.concurrency
            && report.pages.len() + tasks.len() < session.config.max_pages
        {
            let Some(url) = queue.pop_front() else {
                break;
            };
            if !session.robots.allows(&path_query(&url)) {
                report.skipped.push(Skipped {
                    url,
                    reason: "robots_disallowed".into(),
                });
                continue;
            }
            let session = session.clone();
            tasks.spawn(async move { session.page(url).await });
        }
        while let Some(result) = tasks.join_next().await {
            let (page, links) = result?;
            for link in links {
                // Bound the frontier as well as downloaded pages.
                if known.len() >= session.config.max_pages.saturating_mul(20) {
                    report.frontier_truncated = true;
                    break;
                }
                if same_origin(&link, &session.origin)
                    && validate_url(&link).is_ok()
                    && known.insert(link.clone())
                {
                    queue.push_back(link);
                }
            }
            report.pages.push(page);
        }
    }
    report.stopped = *session.stopped.lock().await;
    report.page_limit_reached = report.pages.len() == session.config.max_pages;
    report
        .pages
        .sort_by(|a, b| a.url.as_str().cmp(b.url.as_str()));
    report
        .skipped
        .sort_by(|a, b| a.url.as_str().cmp(b.url.as_str()));
    Ok(report)
}
