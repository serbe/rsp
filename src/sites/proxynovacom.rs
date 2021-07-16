use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let ports = vec!["3128", "80", "8080"];
    let mut list = Vec::new();
    let re = Regex::new(r"title=\D(\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})[\.\-](\d{2,3})\D")?;
    for port in ports {
        let body = crawl(&format!(
            "https://www.proxynova.com/proxy-server-list/port-{}/",
            &port
        ))
        .await?;
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

    #[tokio::test]
    async fn test_proxynovacom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
