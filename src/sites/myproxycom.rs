use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub fn get() -> Result<Vec<String>, RspError> {
    let urls = vec![
        "https://www.my-proxy.com/free-transparent-proxy.html",
        "https://www.my-proxy.com/free-anonymous-proxy.html",
        "https://www.my-proxy.com/free-elite-proxy.html",
        "https://www.my-proxy.com/free-socks-5-proxy.html",
        "https://www.my-proxy.com/free-proxy-list.html",
    ];
    let mut list = Vec::new();
    let re = Regex::new(r"(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3}:\d{2,4})")?;
    for url in urls {
        let body = crawl(url)?;
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
    async fn test_myproxycom() {
        let r = get();
        assert!(r.is_ok());
        assert!(dbg!(r.unwrap().len()) > 0);
    }
}
