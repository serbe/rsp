mod db;
mod netutils;
mod sites;

struct Config {
    db: String,
    target: String,
}

fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    Config { db, target }
}

fn main() {
    let config = get_config();
    let conn = db::get_connection(&config.db);
    
    println!("{:?}", sites::cnproxycom::cnproxycom());
    let ip = netutils::my_ip().unwrap();

    println!("{}", ip);

}
