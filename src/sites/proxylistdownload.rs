use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let schemes = vec!["http", "https", "socks5"];
    let mut list = Vec::new();
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    for scheme in schemes {
        let body = crawl(&format!(
            "https://www.proxy-list.download/api/v1/get?type={}",
            &scheme
        ))
        .await?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| format!("{}://{}", &scheme, cap[1].to_string()))
                .collect(),
        );
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_proxylistdownload() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
