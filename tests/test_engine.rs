use hors::config::SearchEngine;
use hors::engine::search_links;
use reqwest::{Client, ClientBuilder};
use std::str::FromStr;

#[test]
fn test_search_links_with_bing_search_engine() {
    let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
    let client: Client = reqwest::ClientBuilder::new().cookie_store(true).build().unwrap();
    let target_links: Vec<String> = search_links(
        &String::from("how to parse json in rust"),
        search_engine,
        client,
    )
    .unwrap();
    // for search results, what we can do is checking if
    // target_links' host is stackoverflow.com
    assert_ne!(target_links.len(), 0);
    for link in target_links {
        assert!(link.contains("stackoverflow.com"));
    }
}

#[test]
fn test_search_links_with_google_search_engine() {
    let search_engine: SearchEngine = SearchEngine::from_str("google").unwrap();
    let client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
    let target_links: Vec<String> = search_links(
        &String::from("how to parse json in rust"),
        search_engine,
        client,
    )
    .unwrap();
    // for search results, what we can do is checking if
    // target_links' host is stackoverflow.com
    assert_ne!(target_links.len(), 0);
    for link in target_links {
        assert!(link.contains("stackoverflow.com"));
    }
}
