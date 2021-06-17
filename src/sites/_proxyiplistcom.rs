use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let urls = vec![
        "http://proxy-ip-list.com/download/proxy-list-port-3128.txt",
        "http://proxy-ip-list.com/download/free-usa-proxy-ip.txt",
        "http://proxy-ip-list.com/download/free-uk-proxy-list.txt",
        "http://proxy-ip-list.com/download/free-proxy-list.txt",
    ];
    let mut list = Vec::new();
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    for url in urls {
        let body = crawl(url).await?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| cap[1].to_string())
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_proxyiplistcom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
