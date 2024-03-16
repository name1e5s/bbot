use anyhow::{anyhow, Result};
use bitflags::bitflags;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use std::fmt;

use crate::utils::{contain_selector, ToAnyhowError};

#[derive(Debug)]
pub enum Promotion {
    None,
    Free,
    TwoUp,
    TwoUpFree,
    HalfDown,
    TwoUpHalfDown,
    ThirtyPercentDown,
}

impl Promotion {
    pub fn from_str(s: &str) -> Self {
        match s {
            "free_bg" => Self::Free,
            "twoup_bg" => Self::TwoUp,
            "twoupfree_bg" => Self::TwoUpFree,
            "halfdown_bg" => Self::HalfDown,
            "twouphalfdown_bg" => Self::TwoUpHalfDown,
            "thirtypercentdown_bg" => Self::ThirtyPercentDown,
            _ => Self::None,
        }
    }
}

impl Default for Promotion {
    fn default() -> Self {
        Self::None
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Tags: u16 {
        const NEW = 1 << 0;
        const HOT = 1 << 1;
        const CLASSIC = 1 << 2;
        const RECOMMENDED = 1 << 3;
    }
}

impl fmt::Debug for Tags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tags = Vec::new();
        if self.contains(Tags::NEW) {
            tags.push("new");
        }
        if self.contains(Tags::HOT) {
            tags.push("hot");
        }
        if self.contains(Tags::CLASSIC) {
            tags.push("classic");
        }
        if self.contains(Tags::RECOMMENDED) {
            tags.push("recommended");
        }
        if tags.is_empty() {
            tags.push("none")
        }
        write!(f, "{}", tags.join(" | "))
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Default)]
pub struct Torrent {
    pub id: u64,
    pub title: String,
    pub class: String,
    pub sub_class: String,
    pub promotion: Promotion,
    pub tags: Tags,
    pub link: String,
    pub seeding: u64,
    pub downloading: u64,
    pub finished: u64,
    pub size: u64,
}

impl Torrent {
    pub fn from_html(html: &Html) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        let torrents_selector = Selector::parse(".torrents.coltable").anyhow()?;
        let torrents = html
            .select(&torrents_selector)
            .next()
            .ok_or_else(|| anyhow!("Torrents not found"))?;
        let tbody_selector = Selector::parse("tbody").anyhow()?;
        let tbody = torrents
            .select(&tbody_selector)
            .next()
            .ok_or_else(|| anyhow!("Tbody not found"))?;
        for elem in tbody.child_elements().skip(1) {
            let torrent = Self::from_element(elem)?;
            result.push(torrent);
        }
        Ok(result)
    }

    pub fn from_element(element: ElementRef) -> Result<Self> {
        let promotion = element
            .attr("class")
            .map(Promotion::from_str)
            .unwrap_or_default();
        let tags = {
            let mut tags = Tags::empty();
            if contain_selector(".new", element)? {
                tags |= Tags::NEW;
            }
            if contain_selector(".hot", element)? {
                tags |= Tags::HOT;
            }
            if contain_selector(".classic", element)? {
                tags |= Tags::CLASSIC;
            }
            if contain_selector(".recommend", element)? {
                tags |= Tags::RECOMMENDED;
            }
            tags
        };

        let mut children = element.child_elements().skip(1);
        let (class, sub_class) =
            find_class(children.next().ok_or_else(|| anyhow!("class not found"))?)?;
        let (title, link) =
            find_title_and_link(children.next().ok_or_else(|| anyhow!("title not found"))?)?;
        let id: u64 = link
            .split("id=")
            .last()
            .ok_or_else(|| anyhow!("id not found"))?
            .parse()?;

        let mut children = children.skip(2);
        let size = parse_size(
            &children
                .next()
                .ok_or_else(|| anyhow!("size not found"))?
                .inner_html(),
        )?;

        let seeding: u64 = children
            .next()
            .ok_or_else(|| anyhow!("seeding not found"))?
            .text()
            .next()
            .ok_or_else(|| anyhow!("seeding text not found"))?
            .parse()?;
        let downloading: u64 = children
            .next()
            .ok_or_else(|| anyhow!("downloading not found"))?
            .text()
            .next()
            .ok_or_else(|| anyhow!("downloading text not found"))?
            .parse()?;
        let finished: u64 = children
            .next()
            .ok_or_else(|| anyhow!("finished not found"))?
            .text()
            .next()
            .ok_or_else(|| anyhow!("finished text not found"))?
            .parse()?;

        Ok(Torrent {
            id,
            title,
            class,
            sub_class,
            promotion,
            tags,
            link,
            seeding,
            downloading,
            finished,
            size,
        })
    }
}

fn find_class(element: ElementRef) -> Result<(String, String)> {
    let class_selector = Selector::parse(".cat-link").anyhow()?;
    let class = element
        .select(&class_selector)
        .next()
        .ok_or_else(|| anyhow!("cat-link not found"))?
        .text()
        .next()
        .ok_or_else(|| anyhow!("cat-link text not found"))?
        .to_string();
    let sub_class_selector = Selector::parse(".secocat-link").unwrap();
    let sub_class = element
        .select(&sub_class_selector)
        .next()
        .ok_or_else(|| anyhow!("secocat-link not found"))?
        .text()
        .next()
        .ok_or_else(|| anyhow!("secocat-link text not found"))?
        .to_string();
    Ok((class, sub_class))
}

fn find_title_and_link(element: ElementRef) -> Result<(String, String)> {
    let title_selector = Selector::parse("[target=\"_self\"]").anyhow()?;
    let title = element
        .select(&title_selector)
        .next()
        .ok_or_else(|| anyhow!("torrent_name not found"))?
        .attr("title")
        .ok_or_else(|| anyhow!("torrent_name title not found"))?
        .to_string();
    let link_container_selector = Selector::parse(".embedded").anyhow()?;
    let link_container = element
        .select(&link_container_selector)
        .last()
        .ok_or_else(|| anyhow!("link not found"))?;
    let link_selector = Selector::parse("a").anyhow()?;
    let link = link_container
        .select(&link_selector)
        .next()
        .ok_or_else(|| anyhow!("link not found"))?
        .value()
        .attr("href")
        .ok_or_else(|| anyhow!("link href not found"))?
        .to_string();
    Ok((title, link))
}

fn parse_size(s: &str) -> Result<u64> {
    let size = s
        .split("<br>")
        .next()
        .ok_or_else(|| anyhow!("size not found"))?;
    let size = size.parse::<f64>()?;
    let size = size as u64;
    let size = match s {
        s if s.contains("KiB") => size,
        s if s.contains("MiB") => size * 1024_u64,
        s if s.contains("GiB") => size * 1024_u64.pow(2),
        s if s.contains("TiB") => size * 1024_u64.pow(3),
        _ => size,
    };
    Ok(size)
}
