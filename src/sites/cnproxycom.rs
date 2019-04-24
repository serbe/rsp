use crate::netutils::crawl;

use select::document::Document;
use select::predicate::{Name, Predicate};

pub fn cnproxycom() -> Vec<String> {
    let mut ips = Vec::new();
    for link in cnproxycom_links() {
        if let Ok(body) = crawl(&link) {
            for ip in cnproxycom_ips(&body) {
                ips.push(ip);
            }
        }
    }
    ips
}

fn cnproxycom_links() -> Vec<String> {
    let links = vec!["http://cn-proxy.com/".to_string()];
    links
}

fn cnproxycom_ips(body: &str) -> Vec<String> {
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
