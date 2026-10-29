use anyhow::{Context, Result, bail};
use reqwest::{Client, StatusCode};
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use url::Url;

pub const USER_AGENT: &str = "SiteForge/0.1.0 (+https://github.com/skphero1/Siteforge-Rust-Cli-)";
pub fn public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v) => {
            let [a, b, _, _] = v.octets();
            !(v.is_private()
                || v.is_loopback()
                || v.is_link_local()
                || v.is_broadcast()
                || v.is_documentation()
                || v.is_unspecified()
                || a == 0
                || a >= 224
                || (a == 100 && (64..=127).contains(&b))
                || (a == 198 && (b == 18 || b == 19))
                || (a == 192 && b == 0)
                || (a == 192 && b == 88 && v.octets()[2] == 99))
        }
        IpAddr::V6(v) => {
            if let Some(v4) = v.to_ipv4_mapped() {
                return public_ip(IpAddr::V4(v4));
            }
            let s = v.segments();
            // Conservatively permit ordinary global unicast only; exclude special-use ranges.
            (s[0] & 0xe000 == 0x2000)
                && s[0] != 0x2001
                && s[0] != 0x2002
                && !(s[0] == 0x3fff && s[1] < 0x1000)
        }
    }
}
pub fn validate_url(url: &Url) -> Result<()> {
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        bail!("only absolute HTTP(S) URLs are supported");
    }
    if !url.username().is_empty() || url.password().is_some() {
        bail!("URL credentials are forbidden");
    }
    Ok(())
}
pub fn same_origin(a: &Url, b: &Url) -> bool {
    a.origin() == b.origin()
}

#[derive(Clone)]
pub struct Network {
    pub allow_private: bool,
    pub timeout: Duration,
    pub max_bytes: usize,
}
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
    pub location: Option<String>,
    pub html: bool,
}
impl Network {
    pub async fn get(&self, url: &Url) -> Result<Response> {
        validate_url(url)?;
        let host = url
            .host_str()
            .context("missing host")?
            .trim_matches(['[', ']']);
        if !self.allow_private && internal_hostname(host) {
            bail!(
                "internal hostname blocked; use --allow-private only with explicit authorization"
            );
        }
        let port = url.port_or_known_default().context("missing port")?;
        let addresses: Vec<SocketAddr> =
            tokio::time::timeout(self.timeout, tokio::net::lookup_host((host, port)))
                .await
                .context("DNS timeout")??
                .collect();
        if addresses.is_empty() {
            bail!("DNS returned no addresses");
        }
        if !self.allow_private && addresses.iter().any(|a| !public_ip(a.ip())) {
            bail!(
                "non-public destination blocked; --allow-private requires explicit authorization"
            );
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(self.timeout)
            .user_agent(USER_AGENT)
            .resolve_to_addrs(host, &addresses)
            .build()?;
        let mut response = client.get(url.clone()).send().await?;
        let status = response.status();
        let location = response
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let html = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.to_ascii_lowercase().contains("text/html"));
        if matches!(status.as_u16(), 401 | 403 | 429 | 503) {
            return Ok(Response {
                status,
                body: String::new(),
                location,
                html: false,
            });
        }
        if response
            .content_length()
            .is_some_and(|n| n > self.max_bytes as u64)
        {
            bail!("response exceeds body limit");
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if bytes.len() + chunk.len() > self.max_bytes {
                bail!("response exceeds body limit");
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok(Response {
            status,
            body: String::from_utf8_lossy(&bytes).into_owned(),
            location,
            html,
        })
    }
}

fn internal_hostname(host: &str) -> bool {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    if host.parse::<IpAddr>().is_ok() {
        return false;
    }
    !host.contains('.')
        || [
            "localhost",
            "local",
            "internal",
            "home.arpa",
            "invalid",
            "test",
        ]
        .iter()
        .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};
    #[test]
    fn rejects_internal_names() {
        for host in [
            "localhost",
            "LOCALHOST.",
            "service.localhost",
            "router",
            "service.internal",
            "printer.local",
            "router.home.arpa",
        ] {
            assert!(internal_hostname(host));
        }
        assert!(!internal_hostname("example.com"));
        assert!(!internal_hostname("8.8.8.8"));
    }
    #[test]
    fn rejects_special_addresses() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "198.18.0.1",
            "::1",
            "fc00::1",
            "::ffff:127.0.0.1",
            "2001:db8::1",
            "2002:7f00:1::",
        ] {
            assert!(!public_ip(ip.parse().unwrap()), "{ip}");
        }
        assert!(public_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(public_ip(IpAddr::V6(
            "2606:4700:4700::1111".parse::<Ipv6Addr>().unwrap()
        )));
    }
}
