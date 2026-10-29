type Rules = Vec<(bool, String)>;
type Group = (Vec<String>, Rules, f64);

#[derive(Default, Clone)]
pub struct Robots {
    rules: Rules,
    pub delay: f64,
}
impl Robots {
    pub fn parse(text: &str) -> Self {
        let mut groups: Vec<Group> = Vec::new();
        let mut agents = Vec::new();
        let mut rules = Vec::new();
        let mut delay = 0.0_f64;
        let mut directives = false;
        for line in text.trim_start_matches('\u{feff}').lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            let Some((key, value)) = line.split_once(':') else {
                continue;
            };
            let value = value.trim();
            match key.trim().to_ascii_lowercase().as_str() {
                "user-agent" => {
                    if directives {
                        groups.push((
                            std::mem::take(&mut agents),
                            std::mem::take(&mut rules),
                            delay,
                        ));
                        delay = 0.0;
                        directives = false;
                    }
                    agents.push(value.to_ascii_lowercase());
                }
                "allow" | "disallow" if !agents.is_empty() => {
                    directives = true;
                    if !value.is_empty() {
                        rules.push((key.trim().eq_ignore_ascii_case("allow"), normalize(value)));
                    }
                }
                "crawl-delay" if !agents.is_empty() => {
                    directives = true;
                    delay = value
                        .parse::<f64>()
                        .ok()
                        .filter(|v| v.is_finite() && *v >= 0.0)
                        .unwrap_or(0.0);
                }
                _ => {}
            }
        }
        groups.push((agents, rules, delay));
        let specific = groups
            .iter()
            .any(|(a, _, _)| a.iter().any(|v| v == "siteforge"));
        let mut result = Self::default();
        for (agents, rules, delay) in groups {
            if agents
                .iter()
                .any(|v| v == if specific { "siteforge" } else { "*" })
            {
                result.rules.extend(rules);
                result.delay = result.delay.max(delay);
            }
        }
        result
    }
    pub fn allows(&self, path: &str) -> bool {
        let path = normalize(path);
        self.rules
            .iter()
            .filter(|(_, p)| matches(p, &path))
            .max_by_key(|(allow, p)| (p.chars().filter(|c| *c != '*' && *c != '$').count(), *allow))
            .is_none_or(|(allow, _)| *allow)
    }
}
fn matches(pattern: &str, path: &str) -> bool {
    let anchored = pattern.ends_with('$');
    let pattern = pattern.strip_suffix('$').unwrap_or(pattern).as_bytes();
    let path = path.as_bytes();
    let (mut p, mut t) = (0, 0);
    let (mut star, mut retry) = (None, 0);
    while t < path.len() {
        if p == pattern.len() && !anchored {
            return true;
        }
        if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            retry = t;
        } else if p < pattern.len() && pattern[p] == path[t] {
            p += 1;
            t += 1;
        } else if let Some(at) = star {
            retry += 1;
            t = retry;
            p = at + 1;
        } else {
            return false;
        }
    }
    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}
fn normalize(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut result = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Some(v) = (bytes[i + 1] as char)
                .to_digit(16)
                .zip((bytes[i + 2] as char).to_digit(16))
                .map(|(a, b)| (a * 16 + b) as u8)
            {
                if v.is_ascii_alphanumeric() || b"-._~".contains(&v) {
                    result.push(v as char);
                } else {
                    result.push_str(&format!("%{v:02X}"));
                }
                i += 3;
                continue;
            }
        }
        let v = bytes[i];
        if v.is_ascii() {
            result.push(v as char);
        } else {
            result.push_str(&format!("%{v:02X}"));
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn specificity_and_wildcards() {
        let r = Robots::parse(
            "User-agent: *\nDisallow: /\nUser-agent: SiteForge\nDisallow: /private\nAllow: /private/open\nDisallow: /*.pdf$\n",
        );
        assert!(r.allows("/"));
        assert!(!r.allows("/private/a"));
        assert!(r.allows("/private/open"));
        assert!(!r.allows("/a.pdf"));
        assert!(r.allows("/a.pdf?x"));
        assert!(!r.allows("/%70rivate/a"));
        assert!(matches("/*x$", "/xx"));
        assert!(!matches("/a$", "/ab"));
    }
}
