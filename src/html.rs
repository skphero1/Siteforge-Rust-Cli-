use scraper::{Html, Selector};
use serde::Serialize;
use url::Url;
#[derive(Default, Serialize)]
pub struct Seo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub headings: Vec<String>,
    pub canonicals: Vec<String>,
    pub images_missing_alt: usize,
    pub wordpress: bool,
    pub issues: Vec<String>,
}
pub fn analyze(body: &str, base: &Url) -> (Seo, Vec<Url>) {
    let doc = Html::parse_document(body);
    let select = |s: &str| Selector::parse(s).expect("static selector");
    let effective_base = doc
        .select(&select("base[href]"))
        .next()
        .and_then(|e| e.value().attr("href"))
        .and_then(|href| base.join(href).ok())
        .unwrap_or_else(|| base.clone());
    let base = &effective_base;
    let mut seo = Seo::default();
    let titles: Vec<_> = doc
        .select(&select("title"))
        .map(|e| e.text().collect::<String>().trim().to_owned())
        .collect();
    seo.title = titles.first().cloned();
    seo.description = doc
        .select(&select("meta"))
        .find(|e| {
            e.value()
                .attr("name")
                .is_some_and(|n| n.eq_ignore_ascii_case("description"))
        })
        .and_then(|e| e.value().attr("content"))
        .map(str::to_owned);
    seo.headings = doc
        .select(&select("h1, h2, h3, h4, h5, h6"))
        .map(|e| {
            format!(
                "{}: {}",
                e.value().name(),
                e.text().collect::<String>().trim()
            )
        })
        .collect();
    let h1 = doc.select(&select("h1")).count();
    seo.canonicals = doc
        .select(&select("link[rel]"))
        .filter(|e| {
            e.value()
                .attr("rel")
                .unwrap_or("")
                .split_whitespace()
                .any(|v| v.eq_ignore_ascii_case("canonical"))
        })
        .filter_map(|e| e.value().attr("href"))
        .map(str::to_owned)
        .collect();
    seo.images_missing_alt = doc
        .select(&select("img"))
        .filter(|e| e.value().attr("alt").is_none())
        .count();
    let lower = body.to_ascii_lowercase();
    seo.wordpress = lower.contains("wp-content/")
        || lower.contains("wp-includes/")
        || doc.select(&select("meta[name=generator]")).any(|e| {
            e.value()
                .attr("content")
                .is_some_and(|v| v.to_ascii_lowercase().contains("wordpress"))
        });
    for (condition, issue) in [
        (
            seo.title.as_ref().is_none_or(|v| v.is_empty()),
            "missing_title",
        ),
        (titles.len() > 1, "multiple_titles"),
        (
            seo.description.as_ref().is_none_or(|v| v.trim().is_empty()),
            "missing_description",
        ),
        (h1 == 0, "missing_h1"),
        (h1 > 1, "multiple_h1"),
        (seo.canonicals.is_empty(), "missing_canonical"),
        (seo.canonicals.len() > 1, "multiple_canonicals"),
        (seo.images_missing_alt > 0, "missing_alt"),
        (
            seo.canonicals.iter().any(|v| base.join(v).is_err()),
            "invalid_canonical",
        ),
    ] {
        if condition {
            seo.issues.push(issue.into());
        }
    }
    let links = doc
        .select(&select("a[href]"))
        .filter_map(|e| e.value().attr("href"))
        .filter_map(|v| base.join(v).ok())
        .filter(|u| matches!(u.scheme(), "http" | "https"))
        .map(|mut u| {
            u.set_fragment(None);
            u
        })
        .collect();
    (seo, links)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_findings() {
        let (s, links) = analyze(
            include_str!("../tests/fixtures/page.html"),
            &Url::parse("https://example.com").unwrap(),
        );
        assert_eq!(s.title.as_deref(), Some("Fixture"));
        assert_eq!(s.images_missing_alt, 1);
        assert!(s.wordpress);
        assert_eq!(links.len(), 3);
    }
    #[test]
    fn honors_document_base() {
        let (_, links) = analyze(
            "<base href='/docs/'><a href='page'>Page</a>",
            &Url::parse("https://example.com").unwrap(),
        );
        assert_eq!(links[0].as_str(), "https://example.com/docs/page");
    }
    #[test]
    fn empty_alt_is_valid() {
        let (s, _) = analyze("<img alt=''>", &Url::parse("https://example.com").unwrap());
        assert_eq!(s.images_missing_alt, 0);
        assert!(s.issues.contains(&"missing_title".into()));
    }
}
