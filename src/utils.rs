use std::fmt::Debug;

use reqwest::{Client, ClientBuilder};
use scraper::{ElementRef, Selector};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

pub fn client() -> Client {
    ClientBuilder::new()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .build()
        .unwrap()
}

pub async fn rate_limit() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

pub trait ToAnyhowError<T> {
    fn anyhow(self) -> Result<T, anyhow::Error>;
}

impl<T, E> ToAnyhowError<T> for Result<T, E>
where
    E: std::error::Error,
{
    fn anyhow(self) -> Result<T, anyhow::Error> {
        self.map_err(|e| anyhow::anyhow!("{}", e))
    }
}

pub fn contain_selector(selector: &str, elem: ElementRef) -> anyhow::Result<bool> {
    let selector = Selector::parse(selector).anyhow()?;
    Ok(elem.select(&selector).next().is_some())
}
