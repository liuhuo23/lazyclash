use crate::utils::help;
use anyhow::{bail, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use tracing::debug;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PrfItem {
    pub uid: Option<String>,

    /// profile item type
    /// enum value: remote | local | script | merge
    #[serde(rename = "type")]
    pub itype: Option<String>,

    /// profile name
    pub name: Option<String>,

    /// profile file
    pub file: Option<String>,

    /// profile description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// source url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// selected information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,

    /// subscription user info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<PrfExtra>,

    /// updated time
    pub updated: Option<usize>,

    /// profile web page url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home: Option<String>,

    /// the file data
    #[serde(skip)]
    pub file_data: Option<String>,
}

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PrfExtra {
    pub upload: u64,
    pub download: u64,
    pub total: u64,
    pub expire: u64,
}

impl PrfItem {
    /// ## Remote type
    /// create a new item from url
    pub async fn from_url(url: &str) -> Result<PrfItem> {
        let mut builder = reqwest::ClientBuilder::new().use_rustls_tls().no_proxy();

        let version = "clash-verge/unknown".to_string();

        builder = builder.danger_accept_invalid_certs(true);
        builder = builder.user_agent(version);

        let resp = builder.build()?.get(url).send().await?;

        let status_code = resp.status();
        if !StatusCode::is_success(&status_code) {
            bail!("failed to fetch remote profile with status {status_code}")
        }

        let header = resp.headers();
        debug!("header: {:?}", header);
        // parse the Subscription UserInfo
        let extra = match header.get("Subscription-Userinfo") {
            Some(value) => {
                let sub_info = value.to_str().unwrap_or("");
                Some(PrfExtra {
                    upload: help::parse_str(sub_info, "upload").unwrap_or(0),
                    download: help::parse_str(sub_info, "download").unwrap_or(0),
                    total: help::parse_str(sub_info, "total").unwrap_or(0),
                    expire: help::parse_str(sub_info, "expire").unwrap_or(0),
                })
            }
            None => None,
        };
        debug!("extra: {:?}", extra);
        // parse the Content-Disposition
        let filename = match header.get("Content-Disposition") {
            Some(value) => {
                let filename = format!("{value:?}");
                let filename = filename.trim_matches('"');
                match help::parse_str::<String>(filename, "filename*") {
                    Some(filename) => {
                        let iter = percent_encoding::percent_decode(filename.as_bytes());
                        let filename = iter.decode_utf8().unwrap_or_default();
                        filename.split("''").last().map(|s| s.to_string())
                    }
                    None => match help::parse_str::<String>(filename, "filename") {
                        Some(filename) => {
                            let filename = filename.trim_matches('"');
                            Some(filename.to_string())
                        }
                        None => None,
                    },
                }
            }
            None => Some(
                crate::utils::help::get_last_part_and_decode(url).unwrap_or("Remote File".into()),
            ),
        };
        debug!("file_name: {:?}", filename);
        // let update_interval = match header.get("profile-update-interval") {
        //     Some(value) => match value.to_str().unwrap_or("").parse::<u64>() {
        //         Ok(val) => Some(val * 60), // hour -> min
        //         Err(_) => None,
        //     },
        //     None => None,
        // };

        let home = match header.get("profile-web-page-url") {
            Some(value) => {
                let str_value = value.to_str().unwrap_or("");
                Some(str_value.to_string())
            }
            None => None,
        };
        let data = resp.text_with_charset("utf-8").await?;
        debug!("{data}");
        Ok(PrfItem {
            uid: Some(uuid::Uuid::new_v4().to_string()),
            itype: Some("remote".to_string()),
            desc: None,
            name: filename.clone(),
            url: Some(url.to_string()),
            selected: Some(false),
            extra,
            file: filename,
            updated: None,
            home,
            file_data: Some(data),
        })
    }
}

#[cfg(test)]
mod test {
    use super::PrfItem;
    #[tokio::test]
    async fn from_url() -> anyhow::Result<()> {
        let url =
            "https://sub.cloudlion.me/api/v1/client/subscribe?token=6a5a4667da647891b46dc4748422b94c";
        PrfItem::from_url(url).await?;
        Ok(())
    }
}
