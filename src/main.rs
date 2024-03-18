mod config;
mod login;
mod parse;
mod transmission;
mod utils;

use crate::parse::torrent::Torrent;
use crate::parse::user::User;
use anyhow::Result;
use scraper::Html;

const DATA: &str = include_str!("../torrents.php");

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let document = Html::parse_fragment(DATA);
    let user_info = User::from_html(&document)?;
    log::info!("user_info: {:?}", user_info);

    let torrent_infos = Torrent::from_html(&document)?;
    for torrent_info in torrent_infos {
        log::info!("torrent_info: {:?}", torrent_info);
    }
    let trans = &config::config().transmission;
    let client = transmission::Client::new(&trans.url, &trans.user, &trans.password, &trans.dir)?;
    let free_space = client.free_space().await?;
    log::info!("free_space: {}", free_space);
    let torrents = client.get_torrents().await?;
    for torrent in torrents {
        log::info!("torrent: {}", torrent);
    }
    Ok(())
}
