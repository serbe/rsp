use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("https://awmproxy.com/freeproxy.php").map_err(|e| e.to_string())?;
    let re =
        Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| cap[1].to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awmproxycom() {
        assert!(get().is_ok());
    }
}
