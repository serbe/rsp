#![allow(dead_code)]
#![allow(unused_imports)]

use crate::sites::proxynovacom as g;
use db::get_n_old_proxy;

mod db;
//mod netutils;
mod sites;

struct Config {
    db: String,
    target: String,
}

fn post(target: String, data: Vec<String>) {
    let client = reqwest::Client::new();
    let _res = client.post(&target)
        .body(data.join("\n"))
        .send();
}

fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    Config { db, target }
}

fn main() {
    // let config = get_config();
    // let conn = db::get_connection(&config.db);

    // println!("{:?}", sites::cnproxycom::cnproxycom());
    // println!("{:?}", sites::cybersyndromenet::cybersyndromenet());
    // println!("{}", proxydailycom().len());
    // let ip = netutils::my_ip().unwrap();
    // let proxies = get_n_old_proxy(conn, 2000);

    // println!("{}", ip);
    let ips = g::get().unwrap();
    println!("{:?}", ips);
}
