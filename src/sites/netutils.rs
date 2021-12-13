use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::error::RspError;

// pub fn my_ip() -> Result<String, reqwest::Error> {
//     reqwest::get("https://api.ipify.org")?.text()
// }

// pub fn get_p(target: &str) -> Result<String, reqwest::Error> {
//     reqwest::get(target)?.text()
// }

pub fn crawl(link: &str) -> Result<String, RspError> {
    let ua = vec![
	"Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36",
	"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:68.0) Gecko/20100101 Firefox/68.0",
	"Mozilla/5.0 (Windows NT 6.1; WOW64; rv:54.0) Gecko/20100101 Firefox/54.0",
	"Mozilla/5.0 (X11; Linux x86_64; rv:45.0) Gecko/20100101 Thunderbird/45.8.0",
	"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/74.0.3729.157 Safari/537.36",
	"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/534.24 (KHTML, like Gecko) Chrome/11.0.696.3 Safari/534.24",
	"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/605.1.15 (KHTML, like Gecko)"
	];
    let mut rng = thread_rng();
    let rand_ua = *ua.choose(&mut rng).ok_or(RspError::EmptyUserAgent)?;
    let request = ureq::get(link)
        .set("User-Agent", rand_ua)
        .set("Connection", "close")
        .set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .set("Referer", "https://www.google.com/");
    // dbg!(&client);
    let response = request.call()?;
    // dbg!(&response);
    let body = response.into_string()?;
    // dbg!(&body);
    Ok(body)
}
