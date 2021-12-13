use super::netutils::crawl;
use crate::error::RspError;
use regex::Regex;

pub fn get() -> Result<Vec<String>, RspError> {
    let urls = vec![
        "https://www.xicidaili.com/nn/1",
        "https://www.xicidaili.com/nn/2",
        "https://www.xicidaili.com/nn/3",
        "https://www.xicidaili.com/nn/4",
        "https://www.xicidaili.com/nn/5",
        "https://www.xicidaili.com/nn/6",
        "https://www.xicidaili.com/nn/7",
    ];
    let re = Regex::new(r"<td>(\d{2,3}\.\d{2,3}\.\d{2,3}\.\d{2,3})</td>\n<td>(\d{2,5})<")?;
    let mut list = Vec::new();
    for url in urls {
        let body = crawl(url)?;
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
    async fn test_xicidailicom() {
        let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
    }
}
