use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body_url = crawl("http://www.live-socks.net/").await?;
    let re_url = Regex::new(r"href='(http://www.live-socks.net/\d{4}/\d{2}/.+?.html#more)")?;
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
    async fn test_livesocksnet() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
