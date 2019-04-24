pub fn my_ip() -> Result<String, reqwest::Error> {
    reqwest::get("https://api.ipify.org")?.text()
}

pub fn get_p(target: &str) -> Result<String, reqwest::Error> {
    reqwest::get(target)?.text()
}

pub fn crawl(link: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get(link)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0",
        )
        .header("Connection", "close")
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header("Referer", "https://www.google.com/")
        .send()?
        .text()
}
