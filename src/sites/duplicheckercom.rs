use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://www.duplichecker.com/free-proxy-list.php").await?;
    let re = Regex::new(
        r"<div\s.+?>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</div>\n<div\s.+?>(\d{2,4})</div>",
    )?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_duplicheckercom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
