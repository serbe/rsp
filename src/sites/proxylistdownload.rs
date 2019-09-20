use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let schemes = vec!["http", "https", "socks5"];
    let mut list = Vec::new();
    let re =
        Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
    for scheme in schemes {
        let body = crawl(&format!(
            "https://www.proxy-list.download/api/v1/get?type={}",
            &scheme
        ))
        .map_err(|e| e.to_string())?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| format!("{}://{}", &scheme, cap[1].to_string()))
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_proxylistdownload() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
