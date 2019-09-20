use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec![
        "http://www.aliveproxy.com/socks5-list",
        "http://www.aliveproxy.com/high-anonymity-proxy-list",
        "http://www.aliveproxy.com/anonymous-proxy-list",
        "http://www.aliveproxy.com/fastest-proxies",
        "http://www.aliveproxy.com/us-proxy-list",
        "http://www.aliveproxy.com/gb-proxy-list",
        "http://www.aliveproxy.com/fr-proxy-list",
        "http://www.aliveproxy.com/de-proxy-list",
        "http://www.aliveproxy.com/jp-proxy-list",
        "http://www.aliveproxy.com/ca-proxy-list",
        "http://www.aliveproxy.com/ru-proxy-list",
        "http://www.aliveproxy.com/proxy-list-port-80",
        "http://www.aliveproxy.com/proxy-list-port-81",
        "http://www.aliveproxy.com/proxy-list-port-3128",
        "http://www.aliveproxy.com/proxy-list-port-8000",
        "http://www.aliveproxy.com/proxy-list-port-8080",
    ];
    let mut list = Vec::new();
    let re =
        Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
    for url in urls {
        let body = crawl(url).map_err(|e| e.to_string())?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| cap[1].to_string())
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_aliveproxycom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
