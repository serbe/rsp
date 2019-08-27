use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let body = crawl("https://www.proxynova.com/proxy-server-list/port-8080/").map_err(|e| e.to_string())?;
    // super::utils::save("proxynovacom.html", &body);
    let re = Regex::new(r"title=\D(\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})\D").map_err(|e| e.to_string())?;
    Ok(re.captures_iter(&body).map(|cap| format!("{}.{}.{}.{}:8080", &cap[1], &cap[2], &cap[3], &cap[4])).collect())
}