use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let urls = vec![
        "http://www.fakemyip.info/elite-proxies",
        "http://www.fakemyip.info/anonymous-proxies",
        "http://www.fakemyip.info/transparent-proxies",
    ];
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td><td>(\d{2,5})<")?;
    let mut list = Vec::new();
    for url in urls {
        let body = crawl(url).await?;
        list.append(
            &mut re
                .captures_iter(&body)
                .map(|cap| format!("{}:{}", &cap[1], &cap[2]))
                .collect(),
        );
    }
    Ok(list)
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
