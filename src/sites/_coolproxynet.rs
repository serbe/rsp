// use crate::netutils::crawl;

// use select::document::Document;
// use select::predicate::{Name, Predicate};

// pub async fn get() -> Result<Vec<String>, RspError> {
//     let mut ips = Vec::new();
//     for link in coolproxynet_links() {
//         if let Ok(body) = crawl(&link) {
//             for ip in coolproxynet_ips(&body) {
//                 ips.push(ip);
//             }
//         }
//     }
//     Ok(ips)
// }

// fn coolproxynet_links() -> Vec<String> {
//     let links = vec!["https://www.cool-proxy.net/proxies/http_proxy_list/sort:score/direction:desc/page:1".to_string(), "https://www.cool-proxy.net/proxies/http_proxy_list/sort:score/direction:desc/page:2".to_string(), "https://www.cool-proxy.net/proxies/http_proxy_list/sort:score/direction:desc/page:3".to_string(), "https://www.cool-proxy.net/proxies/http_proxy_list/sort:score/direction:desc/page:4".to_string()];
//     links
// }

// fn coolproxynet_ips(body: &str) -> Vec<String> {
//     let mut ips = Vec::new();

//     let document = Document::from(body);

//     for node in document.find(Name("tbody").descendant(Name("tr"))) {
//         match (node.find(Name("td")).nth(0), node.find(Name("td")).nth(1)) {
//             (Some(td1), Some(td2)) => ips.push(format!("http://{}:{}", td1.text(), td2.text())),
//             _ => (),
//         }
//     }
//     ips

// 	var ips []string
// 	r := bytes.NewReader(body)
// 	dom, err := goquery.NewDocumentFromReader(r)
// 	if err != nil {
// 		errmsg("coolProxyNetIPS NewDocumentFromReader", err)
// 		return ips
// 	}
// 	dom.Find("tbody tr").Each(func(_ int, s *goquery.Selection) {
// 		td := s.Find("td")
// 		ip := td.Eq(0).Text()
// 		ip = strings.Replace(ip, `document.write(Base64.decode(str_rot13("`, "", -1)
// 		ip = strings.Replace(ip, `")))`, "", -1)
// 		ip = strings.Map(rot13, ip)
// 		ip = decodeBase64(ip)
// 		port := td.Eq(1).Text()
// 		if len(port) > 1 {
// 			ips = append(ips, "http://"+ip+":"+port)
// 		}
// 	})
// 	return ips
// }
