//! A simple Hacker News API client library based on reqwest and serde.
#![deny(missing_docs)]

use std::time::Duration;

use reqwest::{self, Client};

pub mod types;

use crate::types::Item;

static API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// The API client.
pub struct Api {
    client: Client,
}

impl Api {

    /// Create a new `Api` instance.
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    /// Return a list of top item ids.
    pub fn get_top_items(&self) -> reqwest::Result<Vec<u32>> {
        self.client.get(&format!("{}/topstories.json", API_BASE_URL)).send()?.json()
    }

    /// Return the newest item id.
    ///
    /// To get the 10 latest items, you can decrement the id 10 times.
    pub fn get_latest_item(&self) -> reqwest::Result<u32> {
        self.client.get(&format!("{}/maxitem.json", API_BASE_URL)).send()?.json()
    }

    /// Return the item with the specified id.
    pub fn get_item(&self, id: u32) -> reqwest::Result<Option<Item>> {
        self.client.get(&format!("{}/item/{}.json", API_BASE_URL, id)).send()?.json()
    }
}
