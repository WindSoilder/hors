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
//! use hors::answer;
//! use hors::config::{Config, OutputOption, SearchEngine};
//! use hors::engine;
//! use hors::Result;
//! use reqwest::{Client, ClientBuilder};
//!
//! let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
//! // please make sure that `cookie_store` should set to `true` in client builder.
//! let mut client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
//! let target_links: Vec<String> = engine::search_links(
//!     "how to parse json in rust",
//!     search_engine,
//!     &client
//! ).unwrap();
//! assert_ne!(target_links.len(), 0);
//! for link in target_links {
//!     assert!(link.contains("stackoverflow.com"));
//! }
//! ```
//!
//! Get actual answers according to stackoverflow links.
//!
//! ```rust
//! use hors::answer;
//! use hors::config::{Config, OutputOption};
//! use reqwest::{Client, ClientBuilder};
//!
//! let conf: Config = Config::new(OutputOption::OnlyCode, 3, false);
//! // please make sure that `cookie_store` should set to `true` in client builder.
//! let mut client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
//! let links: Vec<String> = vec![
//!     String::from("https://stackoverflow.com/questions/7771011/how-to-parse-data-in-json")
//! ];
//! let answers: String = answer::get_answers(&links, conf, &client).unwrap();
//! assert!(
//!     answers.contains(
//!         r#"j = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')"#
//!     )
//! );
//! ```

#[macro_use]
extern crate log;

pub mod answer;
pub mod config;
pub mod engine;
mod error;
mod utils;

pub use error::{Error, Result};
