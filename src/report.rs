use crate::crawl::Report;
use anyhow::Result;
use clap::ValueEnum;
#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Terminal,
    Json,
    Csv,
    Html,
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
fn csv(s: &str) -> String {
    // Prevent spreadsheet formula execution when opening untrusted site content.
    let safe = if s.trim_start().starts_with(['=', '+', '-', '@']) {
        format!("'{s}")
    } else {
        s.into()
    };
    format!("\"{}\"", safe.replace('"', "\"\""))
}
pub fn render(report: &Report, format: Format) -> Result<String> {
    match format {
        Format::Json => Ok(serde_json::to_string_pretty(report)?),
        Format::Terminal => {
            let mut text = format!(
                "SiteForge {} — {}\nrobots.txt: {} | sitemap.xml: {:?}: {}\nPages: {} | robots exclusions: {} | stopped: {}\n",
                report.version,
                report.origin,
                report.robots_status,
                report.sitemap_status,
                report.sitemap_note,
                report.pages.len(),
                report.skipped.len(),
                report.stopped
            );
            text.push_str(&format!(
                "Page limit reached: {} | frontier truncated: {}\n",
                report.page_limit_reached, report.frontier_truncated
            ));
            for p in &report.pages {
                text.push_str(&format!(
                    "{} {} | redirects: {} | {} | {}\n",
                    p.status.map_or("ERR".into(), |v| v.to_string()),
                    p.url,
                    p.redirects.len(),
                    p.seo
                        .as_ref()
                        .map_or(String::new(), |s| s.issues.join(", ")),
                    p.error.as_deref().unwrap_or("")
                ));
            }
            // Neutralize control characters from untrusted URLs and errors.
            Ok(text
                .chars()
                .filter(|c| !c.is_control() || *c == '\n')
                .collect())
        }
        Format::Csv => {
            let mut text = "url,status,redirects,title,issues,error\n".to_owned();
            for p in &report.pages {
                let fields = [
                    p.url.to_string(),
                    p.status.map_or(String::new(), |s| s.to_string()),
                    p.redirects
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" -> "),
                    p.seo
                        .as_ref()
                        .and_then(|s| s.title.clone())
                        .unwrap_or_default(),
                    p.seo.as_ref().map_or(String::new(), |s| s.issues.join(";")),
                    p.error.clone().unwrap_or_default(),
                ];
                text.push_str(&fields.iter().map(|s| csv(s)).collect::<Vec<_>>().join(","));
                text.push('\n');
            }
            Ok(text)
        }
        Format::Html => {
            let json = escape(&serde_json::to_string_pretty(report)?);
            Ok(format!(
                "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; style-src 'unsafe-inline'\"><title>SiteForge audit</title><style>body{{font:16px system-ui;max-width:1000px;margin:2rem auto;padding:1rem}}pre{{white-space:pre-wrap;overflow-wrap:anywhere}}</style><h1>SiteForge audit</h1><p>{} pages audited</p><pre>{json}</pre></html>",
                report.pages.len()
            ))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_escaping() {
        assert_eq!(escape("<script>\"&"), "&lt;script&gt;&quot;&amp;");
        assert_eq!(csv("=cmd()"), "\"'=cmd()\"");
        assert_eq!(csv("a\"b"), "\"a\"\"b\"");
    }
}
