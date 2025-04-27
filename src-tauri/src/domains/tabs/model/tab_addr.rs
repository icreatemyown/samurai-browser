use std::str::FromStr;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::WebviewUrl;
use url::Url;

#[derive(Debug, Clone, derive_more::From, Deserialize, Serialize)]
#[serde(tag = "kind", content = "src")]
#[serde(rename_all = "camelCase")]
pub enum TabAddr {
    Url(Url),
    Search(String),
}
impl From<&str> for TabAddr {
    fn from(url: &str) -> Self {
        Url::parse(url)
            .map(TabAddr::Url)
            .unwrap_or_else(|_| Self::Search(url.to_string()))
    }
}

impl From<&TabAddr> for WebviewUrl {
    fn from(val: &TabAddr) -> Self {
        match val {
            TabAddr::Url(url) => WebviewUrl::External(url.clone()),
            TabAddr::Search(s) => {
                let query =
                    Itertools::intersperse(s.split_ascii_whitespace(), "+").collect::<String>();
                let url = format!("https://www.google.com/search?q={query}");
                let url = Url::from_str(&url).expect("Unable to set correct url");

                WebviewUrl::External(url)
            }
        }
    }
}
