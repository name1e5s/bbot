use anyhow::Result;
use chrono::{Local, TimeZone};
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;
use transmission_rpc::{
    types::{BasicAuth, TorrentGetField},
    TransClient,
};

use crate::utils::{Size, ToAnyhowError};

#[derive(Debug)]
pub struct Torrent {
    pub id: u64,
    pub name: String,
    pub total_size: Size,
    pub added_date: i64,
    pub upload_ratio: f64,
}

impl Display for Torrent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time = Local
            .timestamp_opt(self.added_date, 0)
            .single()
            .map(|t| t.to_rfc3339())
            .unwrap_or_else(|| "(invalid)".to_string());
        write!(
            f,
            "id: {}, name: {}, total_size: {}, added_date: {}, upload_ratio: {}",
            self.id, self.name, self.total_size, time, self.upload_ratio
        )
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<Mutex<TransClient>>,
    dir: String,
}

impl Client {
    pub fn new(url: &str, user: &str, password: &str, dir: &str) -> Result<Client> {
        let client = TransClient::with_auth(
            url.parse()?,
            BasicAuth {
                user: user.to_string(),
                password: password.to_string(),
            },
        );
        let client = Arc::new(Mutex::new(client));
        Ok(Client {
            inner: client,
            dir: dir.to_string(),
        })
    }

    pub async fn free_space(&self) -> Result<Size> {
        let free_space = self
            .inner
            .lock()
            .await
            .free_space(self.dir.clone())
            .await
            .anyhow()?;
        Ok(free_space
            .arguments
            .size_bytes
            .try_into()
            .unwrap_or(0)
            .into())
    }

    pub async fn get_torrents(&self) -> Result<Vec<Torrent>> {
        let torrents = self
            .inner
            .lock()
            .await
            .torrent_get(
                Some(vec![
                    TorrentGetField::Id,
                    TorrentGetField::Name,
                    TorrentGetField::TotalSize,
                    TorrentGetField::AddedDate,
                    TorrentGetField::UploadRatio,
                ]),
                None,
            )
            .await
            .anyhow()?;
        let torrents = torrents
            .arguments
            .torrents
            .into_iter()
            .map(|v| {
                let id =
                    v.id.ok_or_else(|| anyhow::anyhow!("id not found"))?
                        .try_into()
                        .unwrap_or(0);
                let name = v.name.ok_or_else(|| anyhow::anyhow!("name not found"))?;
                let total_size: u64 = v
                    .total_size
                    .ok_or_else(|| anyhow::anyhow!("total_size not found"))?
                    .try_into()
                    .unwrap_or(0);
                let total_size = total_size.into();
                let added_date = v
                    .added_date
                    .ok_or_else(|| anyhow::anyhow!("added_date not found"))?;
                let upload_ratio = v
                    .upload_ratio
                    .ok_or_else(|| anyhow::anyhow!("upload_ratio not found"))?
                    .into();
                Ok(Torrent {
                    id,
                    name,
                    total_size,
                    added_date,
                    upload_ratio,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(torrents)
    }
}
