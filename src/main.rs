mod netutils;

fn main() -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("socks5h://192.168.31.1:9050")?;
    let client: reqwest::Client = reqwest::Client::builder().proxy(proxy).build()?;
    let resp = client.get("http://udds.ru:16016/c").send()?.text()?;
    println!("{:#?}", resp);

    let resp = reqwest::get("http://5.39.102.29:16016/c")?.text()?;
    println!("{:#?}", resp);

    let ip = netutils::my_ip()?;
    println!("{}", ip);

    Ok(())
}
