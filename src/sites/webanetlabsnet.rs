use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://webanetlabs.net/publ/24").await?;
    let re_url = Regex::new(r#"href="(/freeproxyweb/proxylist_at_\d{2}\.\d{2}.\d{4}.txt)"#)?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    let mut list = Vec::new();
    let links: Vec<String> = re_url
        .captures_iter(&body)
        .map(|cap| format!("https://webanetlabs.net/{}", &cap[1]))
        .collect();
    dbg!(&links);
    for link in links {
        let body = crawl(&link).await?;
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
    async fn test_webanetlabsnet() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
