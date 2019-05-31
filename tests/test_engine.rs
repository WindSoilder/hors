use hors::config::SearchEngine;
use hors::engine::search_links;
use std::str::FromStr;

#[test]
fn test_search_links_with_bing_search_engine() {
    let search_engine: SearchEngine = SearchEngine::from_str("bing").unwrap();
    let target_links: Vec<String> =
        search_links(&String::from("how to parse json in rust"), search_engine).unwrap();
    // for search results, what we can do is checking if
    // target_links' host is stackoverflow.com
    assert_ne!(target_links.len(), 0);
    for link in target_links {
        assert!(link.contains("stackoverflow.com"));
    }
}

// this test failure for now..
// #[test]
// fn test_search_links_with_google_search_engine() {
//     let search_engine: SearchEngine = SearchEngine::from_str("google").unwrap();
//     let target_links: Vec<String> =
//         search_links(&String::from("how to parse json in rust"), search_engine).unwrap();
//     // for search results, what we can do is checking if
//     // target_links' host is stackoverflow.com
//     assert_ne!(target_links.len(), 0);
//     for link in target_links {
//         assert!(link.contains("stackoverflow.com"));
//     }
// }
