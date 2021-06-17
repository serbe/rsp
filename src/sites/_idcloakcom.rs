use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let body = crawl("http://www.idcloak.com/proxylist/free-us-proxy-list.html").await?;
    let re = Regex::new(r"<td>(\d{2,5})</td><td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td>")?;
    Ok(re
        .captures_iter(&body)
        .map(|cap| format!("{}:{}", &cap[2], &cap[1]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::get;

    #[tokio::test]
    async fn test_idcloakcom() {
        let r = get().await;
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
