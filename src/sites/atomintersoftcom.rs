use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec![
        "http://www.atomintersoft.com/high_anonymity_elite_proxy_list",
        "http://www.atomintersoft.com/anonymous_proxy_list",
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
    fn test_atomintersoftcom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
