use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://www.duplichecker.com/free-proxy-list.php")?;
    dbg!(&body);
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,5})")?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| cap[1].to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_duplicheckercom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
