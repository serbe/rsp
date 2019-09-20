use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec![
        "http://api.foxtools.ru/v2/Proxy.txt?page=1",
        "http://api.foxtools.ru/v2/Proxy.txt?page=2",
        "http://api.foxtools.ru/v2/Proxy.txt?page=3",
        "http://api.foxtools.ru/v2/Proxy.txt?page=4",
        "http://api.foxtools.ru/v2/Proxy.txt?page=5",
        "http://api.foxtools.ru/v2/Proxy.txt?page=6",
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
    fn test_apifoxtoolsru() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
