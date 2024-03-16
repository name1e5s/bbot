use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

use crate::utils::ToAnyhowError;

#[derive(Debug)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn from_html(html: &Html) -> Result<Self> {
        let user_selector = Selector::parse("#info_block").anyhow()?;
        let user = html
            .select(&user_selector)
            .next()
            .ok_or_else(|| anyhow!("User not found"))?;
        let user_selector = Selector::parse(".nowrap").anyhow()?;
        let user_name = user
            .select(&user_selector)
            .next()
            .ok_or_else(|| anyhow!("User name not found"))?
            .text()
            .next()
            .ok_or_else(|| anyhow!("User name text not found"))?
            .to_string();
        Ok(User { name: user_name })
    }
}
