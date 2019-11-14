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
//! use hors::{self, Config, OutputOption, Result, SearchEngine};
//! use reqwest::{Client, ClientBuilder};
//!
//! let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
//! let target_links: Vec<String> = hors::search_links(
//!     "how to parse json in rust",
//!     search_engine,
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
//! use hors::{self, Config, OutputOption};
//! use reqwest::{Client, ClientBuilder};
//!
//! let conf: Config = Config::new(OutputOption::OnlyCode, 3, false);
//! let links: Vec<String> = vec![
//!     String::from("https://stackoverflow.com/questions/7771011/how-to-parse-data-in-json")
//! ];
//! let answers: String = hors::get_answers(&links, conf).unwrap();
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

pub use answer::{get_answers, get_answers_with_client, SPLITTER};
pub use config::{Config, OutputOption, SearchEngine};
pub use engine::{search_links, search_links_with_client};
pub use error::{Error, Result};
