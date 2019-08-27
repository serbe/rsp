use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let urls = vec!["http://proxy-ip-list.com/download/proxy-list-port-3128.txt", "http://proxy-ip-list.com/download/free-usa-proxy-ip.txt", "http://proxy-ip-list.com/download/free-uk-proxy-list.txt", "http://proxy-ip-list.com/download/free-proxy-list.txt"];
    let mut list = Vec::new();
    for url in urls {
        let body = crawl(url).map_err(|e| e.to_string())?;
        let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
        for cap in re.captures_iter(&body) {
            list.push(format!("{}", &cap[1]));
        }
    }
    Ok(list)
}