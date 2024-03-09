use reqwest::{Client, ClientBuilder};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

pub fn client() -> Client {
    ClientBuilder::new().user_agent(USER_AGENT).cookie_store(true).build().unwrap()
}

pub async fn rate_limit() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
