use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://www.socks-proxy.net/").await?;
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td><td>(\d{2,5})<")?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_socksproxynet() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
