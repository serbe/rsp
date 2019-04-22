mod db;
mod netutils;

struct Config {
    db: String,
    target: String
}

fn get_config<'a>() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
        let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
        Config{db, target}
}

fn main() {
    let config = get_config();
    let conn = db::get_connection(&config.db);
    let proxy = reqwest::Proxy::all("socks5h://192.168.31.1:9050").unwrap();
    let client: reqwest::Client = reqwest::Client::builder().proxy(proxy).build().unwrap();
    let resp = client.get("http://udds.ru:16016/c").send().unwrap().text().unwrap();
    println!("{:#?}", resp);

    let resp = reqwest::get("http://5.39.102.29:16016/c").unwrap().text().unwrap();
    println!("{:#?}", resp);

    let ip = netutils::my_ip().unwrap();
    println!("{}", ip);
}
