use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body_url = crawl("http://www.live-socks.net/").map_err(|e| e.to_string())?;
    let re_url = Regex::new(r"href='(http://www.live-socks.net/\d{4}/\d{2}/.+?.html#more)")
        .map_err(|e| e.to_string())?;
    let re =
        Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    for cap_url in re_url.captures_iter(&body_url) {
        let body = crawl(&cap_url[1]).map_err(|e| e.to_string())?;
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
    fn test_livesocksnet() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
