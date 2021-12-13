use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("http://multiproxy.org/txt_all/proxy.txt")?;
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
    async fn test_multiproxyorg() {
        let r = get();
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
