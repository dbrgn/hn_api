//! A simple synchronous Hacker News API (v0) client library based on reqwest
//! and serde.
//!
//! The library currently implements no caching. It simply exposes endpoints as
//! methods.
//!
//! Furthermore, there is no realtime functionality. If you need that, you
//! should probably use a firebase client crate and subscribe to the live
//! endpoints directly.
//!
//! API Docs: <https://github.com/HackerNews/API>
//!
//! ## Usage
//!
//! ```rust
//! use hn_api::nonblocking::HnClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize HTTP client
//!     let api = HnClient::init()
//!         .expect("Could not initialize HN client");
//!
//!     // Fetch latest item
//!     let latest_item_id = api.get_max_item_id().await
//!         .expect("Could not fetch latest item id");
//!     let item = api.get_item(latest_item_id).await
//!         .expect("Could not fetch item");
//!
//!     println!("Latest item: {:?}", item);
//! }
//! ```
//!
//! For an example, see `examples/top.rs`.

#![deny(missing_docs)]

use std::time::Duration;

use futures::future::{join_all, OptionFuture};
use reqwest::{self, Client};

use super::types;

static API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// The API client.
pub struct HnClient {
    client: Client,
}

impl HnClient {
    /// Create a new `HnClient` instance.
    pub fn init() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    /// Return the item with the specified id.
    ///
    /// May return `None` if item id is invalid.
    pub async fn get_item(&self, id: u32) -> reqwest::Result<Option<types::Item>> {
        self.client
            .get(&format!("{}/item/{}.json", API_BASE_URL, id))
            .send()
            .await?
            .json()
            .await
    }

    /// Return the items with the specified ids.
    ///
    /// May return `None` if item id is invalid.
    /// Fails if any of the request failed.
    pub async fn get_items<'a, I>(&self, items: I) -> reqwest::Result<Vec<Option<types::Item>>>
    where
        I: IntoIterator<Item = &'a u32>,
    {
        join_all(items.into_iter().map(|id| self.get_item(*id)))
            .await
            .into_iter()
            .collect()
    }

    /// Return the user with the specified username.
    ///
    /// May return `None` if username is invalid.
    pub async fn get_user(&self, username: String) -> reqwest::Result<Option<types::User>> {
        self.client
            .get(&format!("{}/user/{}.json", API_BASE_URL, username))
            .send()
            .await?
            .json()
            .await
    }

    /// Return all the authors of the specified items.
    ///
    /// May return `None` if item is None or returned user is None.
    /// Fails if any of the request failed.
    pub async fn get_authors(
        &self,
        items: &Vec<Option<types::Item>>,
    ) -> reqwest::Result<Vec<Option<types::User>>> {
        let authors: reqwest::Result<_> = join_all(items.into_iter().map(|item| {
            let a: OptionFuture<_> = item
                .as_ref()
                .and_then(|a| a.author().map(|a| a.to_string()))
                .map(|a| self.get_user(a))
                .into();
            a
        }))
        .await
        .into_iter()
        .map(|a| a.transpose().map(|a| a.flatten()))
        .collect();

        Ok(authors?)
    }

    /// Return the id of the newest item.
    ///
    /// To get the 10 latest items, you can decrement the id 10 times.
    pub async fn get_max_item_id(&self) -> reqwest::Result<u32> {
        self.client
            .get(&format!("{}/maxitem.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return a list of top story item ids.
    pub async fn get_top_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/topstories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return a list of new story item ids.
    pub async fn get_new_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/newstories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return a list of best story item ids.
    pub async fn get_best_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/beststories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return up to 200 latest Ask HN story item ids.
    pub async fn get_ask_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/askstories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return up to 200 latest Show HN story item ids.
    pub async fn get_show_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/showstories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return up to 200 latest Job story item ids.
    pub async fn get_job_stories(&self) -> reqwest::Result<Vec<u32>> {
        self.client
            .get(&format!("{}/jobstories.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }

    /// Return a list of items and users that have been updated recently.
    pub async fn get_updates(&self) -> reqwest::Result<types::Updates> {
        self.client
            .get(&format!("{}/updates.json", API_BASE_URL))
            .send()
            .await?
            .json()
            .await
    }
}
