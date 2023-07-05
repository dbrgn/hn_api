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
//! use hn_api::blocking::HnClient;
//!
//! // Initialize HTTP client
//! let api = HnClient::init()
//!     .expect("Could not initialize HN client");
//!
//! // Fetch latest item
//! let latest_item_id = api.get_max_item_id()
//!     .expect("Could not fetch latest item id");
//! let item = api.get_item(latest_item_id)
//!     .expect("Could not fetch item");
//!
//! println!("Latest item: {:?}", item);
//! ```
//!
//! For an example, see `examples/top.rs`.

#![deny(missing_docs)]

use std::time::Duration;

use super::{types, HnClientError, Result};

static API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// The API client.
pub struct HnClient {
    client: reqwest::blocking::Client,
}

impl HnClient {
    /// Create a new `HnClient` instance.
    pub fn init() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    /// Return the item with the specified id.
    ///
    /// May return `None` if item id is invalid.
    pub fn get_item(&self, id: u32) -> Result<Option<types::Item>> {
        self.client
            .get(&format!("{}/item/{}.json", API_BASE_URL, id))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return the user with the specified username.
    ///
    /// May return `None` if username is invalid.
    pub fn get_user(&self, username: &str) -> Result<Option<types::User>> {
        self.client
            .get(&format!("{}/user/{}.json", API_BASE_URL, username))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return the id of the newest item.
    ///
    /// To get the 10 latest items, you can decrement the id 10 times.
    pub fn get_max_item_id(&self) -> Result<u32> {
        self.client
            .get(&format!("{}/maxitem.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return a list of top story item ids.
    pub fn get_top_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/topstories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return a list of new story item ids.
    pub fn get_new_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/newstories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return a list of best story item ids.
    pub fn get_best_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/beststories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return up to 200 latest Ask HN story item ids.
    pub fn get_ask_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/askstories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return up to 200 latest Show HN story item ids.
    pub fn get_show_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/showstories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return up to 200 latest Job story item ids.
    pub fn get_job_stories(&self) -> Result<Vec<u32>> {
        self.client
            .get(&format!("{}/jobstories.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }

    /// Return a list of items and users that have been updated recently.
    pub fn get_updates(&self) -> Result<types::Updates> {
        self.client
            .get(&format!("{}/updates.json", API_BASE_URL))
            .send()?
            .json()
            .map_err(HnClientError::from)
    }
}
