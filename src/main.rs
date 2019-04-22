mod db;
mod netutils;

struct Config {
    db: String,
    target: String
}

fn get_config<'a>() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db_uri like postgres://postgres@localhost:5433 in environment");
        let target = dotenv::var("db")
        .expect("No found variable target like http://targethost:433/path in environment");
        Config{db, target}
}

fn main() -> Result<(), reqwest::Error> {
    let config = get_config();
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
