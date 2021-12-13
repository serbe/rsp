use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("https://webanetlabs.net/publ/24")?;
    let re_url = Regex::new(r#"href="(/proxylist.*\.html)"#)?;
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    let mut list = Vec::new();
    let links: Vec<String> = re_url
        .captures_iter(&body)
        .map(|cap| format!("https://webanetlabs.net/{}", &cap[1]))
        .collect();
    for link in links {
        let body = crawl(&link)?;
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
        let r = get();
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
