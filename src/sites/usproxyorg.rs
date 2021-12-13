use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://us-proxy.org/")?;
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
    async fn test_usproxyorg() {
        let r = get();
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
