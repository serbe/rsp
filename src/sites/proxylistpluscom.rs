pub fn get() {
    let names = vec!["Fresh-HTTP-Proxy", "SSL", "Socks"];
    let urls = (1..8).map(|page| names.iter().map(|name| format!("http://list.proxylistplus.com/{}-List-{}", name.clone(), page.clone())));
    println!("{:?}", urls);
}
