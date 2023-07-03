use regex::Regex;

use super::netutils::crawl;
use super::utils::save;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://checkerproxy.net")?;
    let re_url = Regex::new(r#"href="(/freeproxyweb/proxylist_at_\d{2}\.\d{2}.\d{4}.txt)"#)?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    let mut list = Vec::new();
    let links: Vec<String> = re_url
        .captures_iter(&body)
        .map(|cap| format!("https://webanetlabs.net/{}", &cap[1]))
        .collect();
    for link in links {
        let body = crawl(&link)?;
        for cap in re.captures_iter(&body) {
            list.push(cap[1].to_string());
        }
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_checkerproxynet() {
        let body = crawl("https://checkerproxy.net").unwrap();
        save("checkerproxynet.html", &body);
        // let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
        assert!(true);
    }
}
