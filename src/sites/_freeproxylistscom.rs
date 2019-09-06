use regex::Regex;

pub fn get() {
    let domain = "freeproxylists.com";

        // let exp = Regex::new(r"href\s*=\s*['"](?P<t>[^'"]*)/(?P<uts>\d{10})[^'"]*['"]").unnwrap();
        let urls = vec![
            "http://www.freeproxylists.com/socks.html",
            "http://www.freeproxylists.com/elite.html",
            "http://www.freeproxylists.com/anonymous.html",
        ];
        // pages = await asyncio.gather(*[self.get(url) for url in urls])
        // params = re.findall(exp, ''.join(pages))
        // tpl = 'http://www.freeproxylists.com/load_{}_{}.html'
        // # example: http://www.freeproxylists.com/load_socks_1448724717.html
        // urls = [tpl.format(t, uts) for t, uts in params]
        // await self._find_on_pages(urls)
}