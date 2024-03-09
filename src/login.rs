use crate::{config, utils};
use anyhow::Result;
use reqwest::Client;

fn login_url() -> String {
    format!("{}/login.php", config::config().site.base_url)
}

fn take_login_url() -> String {
    format!("{}/takelogin.php", config::config().site.base_url)
}

async fn login_action(client: &Client) -> Result<()> {
    // visit login.php at first
    let _ = client.get(&login_url()).send().await?;
    // then post to takelogin.php
    let resp = client.post(&take_login_url())
        .form(&[
            ("logintype", "username"),
            ("autologin", "yes"),
            ("userinput", &config::config().site.username),
            ("password", &config::config().site.password),
        ])
        .send().await?;
    let body = resp.text().await?;
    if body.contains("最近消息") {
        Ok(())
    } else {
        Err(anyhow::anyhow!("login failed"))
    }
}

pub async fn login(client: &Client) -> Result<()> {
    // retry 5 times
    for _ in 0..4 {
        match login_action(client).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                log::error!("login failed: {}", e);
            }
        }
        utils::rate_limit().await;
    }
    login_action(client).await
}