// need js
use regex::Regex;

use super::netutils::crawl;
use super::utils::save;

pub fn get() -> Result<Vec<String>, String> {
        // let exp = Regex::new(r"href\s*=\s*['"](?P<t>[^'"]*)/(?P<uts>\d{10})[^'"]*['"]").unnwrap();
        let urls = vec![
            "http://www.freeproxylists.com/socks.html",
            "http://www.freeproxylists.com/elite.html",
            "http://www.freeproxylists.com/anonymous.html",
        ];
        let mut list = Vec::new();
        let re_url = Regex::new(r#"href\s*=\s*['"]([^'"]*/\d{10})[^'"]*['"]"#).map_err(|e| e.to_string())?;
        for url in urls {
            let body = crawl(url).map_err(|e| e.to_string())?;
            let links: Vec<String> = re_url
                .captures_iter(&body)
                .map(|cap| format!("http://www.freeproxylists.com/{}", &cap[1]))
                .collect();
            for link in links {

            }
        }
        // pages = await asyncio.gather(*[self.get(url) for url in urls])
        // params = re.findall(exp, ''.join(pages))
        // tpl = 'http://www.freeproxylists.com/load_{}_{}.html'
        // # example: http://www.freeproxylists.com/load_socks_1448724717.html
        // urls = [tpl.format(t, uts) for t, uts in params]
        // await self._find_on_pages(urls)
        Ok(list)
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_freeproxylistscom() {
        let body = crawl("http://www.freeproxylists.com/socks/d1568968223.html").unwrap();
        let r = save("freeproxylistscom.html", &body);
        println!("{:?}", r);
        // let r = get();
        assert!(r.is_ok());
        assert!(r.unwrap().len() > 0);
        assert!(true);
    }
}
