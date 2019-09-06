use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("http://proxy-daily.com/").map_err(|e| e.to_string())?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})").map_err(|e| e.to_string())?;
    Ok(re.captures_iter(&body).map(|cap| format!("{}", &cap[1])).collect())
}