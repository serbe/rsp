use netc::Client;

use error::RspError;

mod client;
mod codec;
mod error;
mod sites;

async fn post(target: &str, data: Vec<String>) -> Result<(), RspError> {
    let mut client = Client::builder()
        .post(target)
        .body(data.join("\n"))
        .build()
        .await?;
    client.send().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), RspError> {
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    post(&target, sites::ab57ru::get().await?).await?;
    post(&target, sites::aliveproxycom::get().await?).await?;
    post(&target, sites::apifoxtoolsru::get().await?).await?;
    post(&target, sites::atomintersoftcom::get().await?).await?;
    post(&target, sites::awmproxycom::get().await?).await?;
    post(&target, sites::cnproxycom::get().await?).await?;
    post(&target, sites::duplicheckercom::get().await?).await?;
    post(&target, sites::fakemyipinfo::get().await?).await?;
    post(&target, sites::freeproxylistcom::get().await?).await?;
    post(&target, sites::freeproxylistnet::get().await?).await?;
    post(&target, sites::givemeproxycom::get().await?).await?;
    post(&target, sites::globalproxiesblogspotcom::get().await?).await?;
    post(&target, sites::httptunnelge::get().await?).await?;
    post(&target, sites::idcloakcom::get().await?).await?;
    post(&target, sites::livesocksnet::get().await?).await?;
    post(&target, sites::mrhinkydinkcom::get().await?).await?;
    post(&target, sites::multiproxyorg::get().await?).await?;
    post(&target, sites::myproxycom::get().await?).await?;
    post(&target, sites::openproxyspace::get().await?).await?;
    post(&target, sites::proxycenterblognet::get().await?).await?;
    post(&target, sites::proxydailycom::get().await?).await?;
    post(&target, sites::proxyiplistcom::get().await?).await?;
    post(&target, sites::proxylistdailynet::get().await?).await?;
    post(&target, sites::proxylistdownload::get().await?).await?;
    post(&target, sites::proxylistme::get().await?).await?;
    post(&target, sites::proxylistsnet::get().await?).await?;
    post(&target, sites::proxynovacom::get().await?).await?;
    post(&target, sites::rmccurdycom::get().await?).await?;
    post(&target, sites::socksproxynet::get().await?).await?;
    post(&target, sites::sslproxiesorg::get().await?).await?;
    post(&target, sites::usproxyorg::get().await?).await?;
    post(&target, sites::webanetlabsnet::get().await?).await?;
    post(&target, sites::xicidailicom::get().await?).await?;
    Ok(())
}
