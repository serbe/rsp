use error::RspError;
use netc::Client;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

// mod client;
// mod codec;
mod error;
mod sites;

static TARGET: Lazy<String> = Lazy::new(get_env);

fn get_env() -> String {
    dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment")
}

async fn post(data: Vec<String>) -> Result<(), RspError> {
    Client::builder()
        .post(TARGET.as_str())
        .body(data.join("\n"))
        .build()
        .await?
        .send()
        .await?;
    Ok(())
}

async fn run() -> Result<(), RspError> {
    post(sites::ab57ru::get().await?).await?;
    post(sites::aliveproxycom::get().await?).await?;
    // post(sites::apifoxtoolsru::get().await?)?;
    post(sites::atomintersoftcom::get().await?).await?;
    // post(sites::awmproxycom::get().await?)?;
    // post(sites::_cnproxycom::get().await?)?;
    // post(sites::duplicheckercom::get().await?)?;
    post(sites::fakemyipinfo::get().await?).await?;
    post(sites::freeproxylistcom::get().await?).await?;
    post(sites::freeproxylistnet::get().await?).await?;
    // post(sites::_givemeproxycom::get().await?)?;
    post(sites::globalproxiesblogspotcom::get().await?).await?;
    // post(sites::httptunnelge::get().await?).await?;
    // post(sites::_idcloakcom::get().await?)?;
    // post(sites::livesocksnet::get().await?)?;
    // post(sites::mrhinkydinkcom::get().await?)?;
    post(sites::multiproxyorg::get().await?).await?;
    post(sites::myproxycom::get().await?).await?;
    // post(sites::openproxyspace::get().await?)?;
    post(sites::proxycenterblognet::get().await?).await?;
    post(sites::proxydailycom::get().await?).await?;
    // post(sites::_proxyiplistcom::get().await?)?;
    // post(sites::_proxylistdailynet::get().await?)?;
    post(sites::proxylistdownload::get().await?).await?;
    // post(sites::proxylistsnet::get().await?)?;
    // post(sites::proxynovacom::get().await?).await?;
    post(sites::rmccurdycom::get().await?).await?;
    post(sites::socksproxynet::get().await?).await?;
    post(sites::sslproxiesorg::get().await?).await?;
    post(sites::usproxyorg::get().await?).await?;
    post(sites::webanetlabsnet::get().await?).await?;
    // post(sites::_xicidailicom::get().await?)?;
    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        run().await.unwrap();
    })
}
