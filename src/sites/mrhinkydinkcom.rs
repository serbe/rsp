use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("http://www.mrhinkydink.com/proxies.htm").map_err(|e| e.to_string())?;
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td>\n<td>(\d{2,5})<")
        .map_err(|e| e.to_string())?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mrhinkydinkcom() {
        assert!(get().is_ok());
    }
}
