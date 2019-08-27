use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("https://www.duplichecker.com/free-proxy-list.php").map_err(|e| e.to_string())?;
    // super::utils::save("duplicheckercom.html", &body);
    let re = Regex::new(r"<div\s.+?>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</div>\n<div\s.+?>(\d{2,4})</div>").map_err(|e| e.to_string())?;
    Ok(re.captures_iter(&body).map(|cap| format!("{}:{}", &cap[1], &cap[2])).collect())
}