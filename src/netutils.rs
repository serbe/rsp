pub fn my_ip() -> Result<String, reqwest::Error> {
    reqwest::get("https://api.ipify.org")?.text()
}

pub fn get_p(target: &str) -> Result<String, reqwest::Error> {
    reqwest::get(target)?.text()
}