use super::netutils::crawl;
use regex::Regex;

pub fn get() -> Result<Vec<String>, String> {
    let ports = vec!["3128", "80", "8080"];
    let mut list = Vec::new();
    let re = Regex::new(r"title=\D(\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})\D")
        .map_err(|e| e.to_string())?;
    for port in ports {
        let body = crawl(&format!(
            "https://www.proxynova.com/proxy-server-list/port-{}/",
            &port
        ))
        .map_err(|e| e.to_string())?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| format!("{}.{}.{}.{}:{}", &cap[1], &cap[2], &cap[3], &cap[4], port))
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_proxynovacom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
