use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec![
        "https://free-proxy-list.com/?page=1",
        "https://free-proxy-list.com/?page=2",
        "https://free-proxy-list.com/?page=3",
        "https://free-proxy-list.com/?page=4",
        "https://free-proxy-list.com/?page=5",
        "https://free-proxy-list.com/?search=1&page=&port=&type%5B%5D=http&type%5B%5D=https&level%5B%5D=high-anonymous&speed%5B%5D=2&speed%5B%5D=3&connect_time%5B%5D=3&up_time=60&search=Search",
    ];
    let re =
        Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,5})").map_err(|e| e.to_string())?;
    let mut list = Vec::new();
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
    fn test_freeproxylistcom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
