use regex::Regex;

use super::netutils::crawl;
use super::utils::save;

pub fn get() -> Result<Vec<String>, String> {
        let body = crawl("https://checkerproxy.net").map_err(|e| e.to_string())?;
        let re_url = Regex::new(r#"href="(/freeproxyweb/proxylist_at_\d{2}\.\d{2}.\d{4}.txt)"#).map_err(|e| e.to_string())?;
        let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
        let mut list = Vec::new();
        let links: Vec<String> = re_url
            .captures_iter(&body)
            .map(|cap| format!("https://webanetlabs.net/{}", &cap[1]))
            .collect();
        for link in links {
            let body = crawl(&link).map_err(|e| e.to_string())?;
            for cap in re.captures_iter(&body) {
                list.push(cap[1].to_string());
            }
        }
        Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_checkerproxynet() {
        let body = crawl("https://checkerproxy.net").unwrap();
        save("checkerproxynet.html", &body);
        // let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
        assert!(true);
    }
}
