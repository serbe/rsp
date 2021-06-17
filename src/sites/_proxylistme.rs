use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let urls = vec![
        "https://proxylist.me/?page=1",
        "https://proxylist.me/?page=2",
        "https://proxylist.me/?page=3",
        "https://proxylist.me/?page=4",
    ];
    let re = Regex::new(
        r#"href=.*?>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</a></td>\n.*\n.*<td\s+>(\d{2,5})<"#,
    )?;
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
    async fn test_proxylistme() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
