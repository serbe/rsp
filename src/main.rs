mod client;
mod codec;
mod sites;

fn post(target: &str, data: Result<Vec<String>, String>) {
    if let Ok(list) = data {
        let client = reqwest::Client::new();
        let _ = client.post(target).body(list.join("\n")).send();
    }
}

fn main() {
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    post(&target, sites::ab57ru::get());
    post(&target, sites::aliveproxycom::get());
    post(&target, sites::apifoxtoolsru::get());
    post(&target, sites::atomintersoftcom::get());
    post(&target, sites::awmproxycom::get());
    post(&target, sites::cnproxycom::get());
    post(&target, sites::duplicheckercom::get());
    post(&target, sites::fakemyipinfo::get());
    post(&target, sites::freeproxylistcom::get());
    post(&target, sites::freeproxylistnet::get());
    post(&target, sites::givemeproxycom::get());
    post(&target, sites::globalproxiesblogspotcom::get());
    post(&target, sites::httptunnelge::get());
    post(&target, sites::idcloakcom::get());
    post(&target, sites::livesocksnet::get());
    post(&target, sites::mrhinkydinkcom::get());
    post(&target, sites::multiproxyorg::get());
    post(&target, sites::myproxycom::get());
    post(&target, sites::openproxyspace::get());
    post(&target, sites::proxycenterblognet::get());
    post(&target, sites::proxydailycom::get());
    post(&target, sites::proxyiplistcom::get());
    post(&target, sites::proxylistdailynet::get());
    post(&target, sites::proxylistdownload::get());
    post(&target, sites::proxylistme::get());
    post(&target, sites::proxylistsnet::get());
    post(&target, sites::proxynovacom::get());
    post(&target, sites::rmccurdycom::get());
    post(&target, sites::socksproxynet::get());
    post(&target, sites::sslproxiesorg::get());
    post(&target, sites::usproxyorg::get());
    post(&target, sites::webanetlabsnet::get());
    post(&target, sites::xicidailicom::get());
}
