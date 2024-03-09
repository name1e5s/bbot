mod utils;
mod config;
mod login;

use anyhow::Result;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let client = utils::client();
    let _ = login::login(&client).await?;
    // get torrents.php
    let resp = client.get(&format!("{}/torrents.php", config::config().site.base_url)).send().await?;
    let body = resp.text().await?;
    log::info!("{}", body);
    log::info!("Hello, world!");
    Ok(())
}
