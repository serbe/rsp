use crate::sites::netutils::crawl;
use crate::sites::utils::save;

use select::document::Document;
use select::predicate::{Name, Predicate};

pub fn proxydailycom() -> Vec<String> {
    let mut ips = Vec::new();
    for link in proxydailycom_links() {
        dbg!(&link);
        match crawl(&link) {
            Ok(body) => {
            save("proxydailycom.txt", &body);
            for ip in proxydailycom_ips(&body) {
                ips.push(ip);
            }
        },
            Err(e) => {
                dbg!(e);
            },
        }
    }
    ips
}

fn proxydailycom_links() -> Vec<String> {
    let links = vec!["http://proxy-daily.com/".to_string()];
    links
}

fn proxydailycom_ips(body: &str) -> Vec<String> {
    let mut ips = Vec::new();

    let document = Document::from(body);

    for node in document.find(Name("tbody").descendant(Name("tr"))) {
        if let (Some(td1), Some(td2)) = (node.find(Name("td")).nth(0), node.find(Name("td")).nth(1))
        {
            ips.push(format!("http://{}:{}", td1.text(), td2.text()))
        }
    }
    ips
}
