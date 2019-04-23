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
        match (node.find(Name("td")).nth(0), node.find(Name("td")).nth(1)) {
            (Some(td1), Some(td2)) => ips.push(format!("http://{}:{}", td1.text(), td2.text())),
            _ => ()
        }
    }
    // r := bytes.NewReader(body)
    // dom, err := goquery.NewDocumentFromReader(r)
    // if err != nil {
    // 	errmsg("cnProxyCom NewDocumentFromReader", err)
    // 	return ips
    // }
    // dom.Find("tbody tr").Each(func(_ int, s *goquery.Selection) {
    // 	td := s.Find("td")
    // 	ips = append(ips, "http://"+td.Eq(0).Text()+":"+td.Eq(1).Text())
    // })
    ips
}