use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec![
        "http://www.fakemyip.info/elite-proxies",
        "http://www.fakemyip.info/anonymous-proxies",
        "http://www.fakemyip.info/transparent-proxies",
    ];
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td><td>(\d{2,5})<")
        .map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    for url in urls {
        let body = crawl(url).map_err(|e| e.to_string())?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_duplicheckercom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
