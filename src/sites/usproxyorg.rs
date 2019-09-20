use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("https://us-proxy.org/").map_err(|e| e.to_string())?;
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td><td>(\d{2,5})<")
        .map_err(|e| e.to_string())?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_usproxyorg() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
