use super::utils::save;
use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("http://givemeproxy.com/").map_err(|e| e.to_string())?;
    let re = Regex::new(r"<td\s.+?>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td><td.+?>(\d{2,5})<").map_err(|e| e.to_string())?;
    Ok(re.captures_iter(&body).map(|cap| format!("{}:{}", &cap[1], &cap[2])).collect())
}