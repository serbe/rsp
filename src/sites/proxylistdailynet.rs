use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body_url = crawl("https://www.proxylistdaily.net/").await?;
    let re_url = Regex::new(r"(https://www.proxylistdaily.net/p/\w.+?\d{4,6}.html)")?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    let mut list = Vec::new();
    for cap_url in re_url.captures_iter(&body_url) {
        let body = crawl(&cap_url[1]).await?;
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
    async fn test_proxylistdailynet() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
