use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://www.rmccurdy.com/scripts/proxy/good.txt").await?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| cap[1].to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_rmccurdycom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
