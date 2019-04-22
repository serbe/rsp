use chrono::{Duration, NaiveDateTime};
use postgres::{Connection, TlsMode};

pub struct Proxy {
	insert: bool,
	update: bool,
	work: bool,
	anon: bool,
	checks: u64,
	hostname: String,
	host: String,
	port: String,
	scheme: String,
	create_at: NaiveDateTime,
	update_at: NaiveDateTime,
	response: Duration,
}

pub fn get_connection(params: &str) -> Connection {
    Connection::connect(params, TlsMode::None).unwrap()
}

