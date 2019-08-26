use super::utils::save;
use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("http://www.idcloak.com/proxylist/free-us-proxy-list.html").map_err(|e| e.to_string())?;
    let re = Regex::new(r"<td>(\d{2,5})</td><td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td>").map_err(|e| e.to_string())?;
    Ok(re.captures_iter(&body).map(|cap| format!("{}:{}", &cap[2], &cap[1])).collect())
}