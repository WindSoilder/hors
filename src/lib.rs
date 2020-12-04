//! Instant coding answers lib.  Which provides two main features:
//!
//! 1. get stackoverflow links according to user input query.
//! 2. get actual answers according to stackoverflow links.
//!
//! # Examples
//!
//! Get stackoverflow links according to user input query.
//!
//! ```rust
//! use std::str::FromStr;
//! use hors::{self, SearchEngine};
//!
//! # async fn run() {
//! let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
//! let target_links: Vec<String> = hors::search_links(
//!     "how to parse json in rust",
//!     search_engine,
//! )
//! .await
//! .unwrap();
//! assert_ne!(target_links.len(), 0);
//! for link in target_links {
//!     assert!(link.contains("stackoverflow.com"));
//! }
//! # }
//! ```
//!
//! Get actual answers according to stackoverflow links.
//!
//! ```rust
//! use hors::{self, Config, OutputOption};
//!
//! # async fn run() {
//! let conf: Config = Config::new(OutputOption::OnlyCode, 3, false);
//! let links: Vec<String> = vec![
//!     String::from("https://stackoverflow.com/questions/7771011/how-to-parse-data-in-json")
//! ];
//! let answers: String = hors::get_answers(&links, conf).await.unwrap();
//! assert!(
//!     answers.contains(
//!         r#"data = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')"#
//!     )
//! );
//! # }
//! ```
//!
//! # Advanced usage:
//! Calling `get_answers` or `search_links` will make a new connection through network, if you want
//! to make use of connection pool, please use `get_answers_with_client` and `search_links_with_client`.
//!
//! In this way, all you need to do is initialize a `reqwest::Client` through `reqwest::ClientBuilder`,
//! just remember to set cookie_store on `ClientBuilder` to true.
//!
//! # Examples
//!
//! ```rust
//! use std::str::FromStr;
//! use hors::{self, Config, OutputOption, SearchEngine};
//! use reqwest::{Client, ClientBuilder};
//!
//! # async fn run() {
//! let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
//! // please make sure that `cookie_store` should set to `true` in client builder.
//! let mut client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
//! let target_links: Vec<String> = hors::search_links_with_client(
//!     "how to parse json in rust",
//!     search_engine,
//!     &client
//! )
//! .await
//! .unwrap();
//! assert_ne!(target_links.len(), 0);
//! for link in target_links {
//!     assert!(link.contains("stackoverflow.com"));
//! }
//! # }
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub mod answer;
pub mod config;
pub mod engine;

mod error;
mod output;
mod search_config;
mod utils;

pub use answer::{clear_local_cache, get_answers, get_answers_with_client, SPLITTER};
pub use config::{Config, OutputOption, PagingOption, SearchEngine};
pub use engine::{search_links, search_links_with_client};
pub use error::{Error, Result};
pub use output::Output;
