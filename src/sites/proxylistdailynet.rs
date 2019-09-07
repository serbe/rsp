use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body_url = crawl("https://www.proxylistdaily.net/").map_err(|e| e.to_string())?;
    let re_url = Regex::new(r"(https://www.proxylistdaily.net/p/\w.+?\d{4,6}.html)")
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
    use super::*;

    #[test]
    fn test_proxylistdailynet() {
        assert!(get().is_ok());
    }
}
