mod config;
mod login;
mod parse;
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
    Ok(())
}
