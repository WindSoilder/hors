mod bing;
mod google;

use crate::config::SearchEngine;
use crate::error::{HorsError, Result};
use crate::utils::random_agent;
use reqwest::RequestBuilder;

/// Search result links under the `bing` search engine.
///
/// This function will go through network to find out useful links in bing.
///
/// # Arguments
///
/// * `query` - The user input query String.
/// * `search_engine` - indicate which search engine we use to search result links.
///
/// # Returns
///
/// If search links successfully, it will return a Vector of String, which indicate
/// relative links to got answer.  Else return an Error.
pub fn search_links(query: &String, search_engine: SearchEngine) -> Result<Vec<String>> {
    let fetch_url: String = get_query_url(query, &search_engine);
    let page: String = fetch(&fetch_url)?;
    let extract_results = extract_links(&page, &search_engine);
    match extract_results {
        Some(links) => return Ok(links),
        None => {
            return Err(HorsError::from_parse("Can't find search result..."));
        }
    }
}

fn get_query_url(query: &String, search_engine: &SearchEngine) -> String {
    match search_engine {
        SearchEngine::Bing => return bing::get_query_url(query),
        SearchEngine::Google => return google::get_query_url(query),
    }
}

/// Fetch actual page according to given url.
///
/// # Arguments
///
/// * `search_url` - The url which should lead to search result page.
///
/// # Returns
///
/// If get search result page successfully, it will return the content of page,
/// or returns error.
fn fetch(search_url: &String) -> Result<String> {
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    let request: RequestBuilder = client
        .get(search_url.as_str())
        .header(reqwest::header::USER_AGENT, random_agent());
    debug!("Request to bing information: {:?}", request);
    let mut res = request.send()?;
    let page: String = res.text()?;
    return Ok(page);
}

/// Extract links from given page.
///
/// # Arguments
///
/// * `page` - the search result page, which is mainly got by `fetch` function.
/// * `search_engine` - indicate which search engine we can use to extract links out.
///
/// # Returns
///
/// Links to the relative question, or returns None if we can't find it.
fn extract_links(page: &String, search_engine: &SearchEngine) -> Option<Vec<String>> {
    match search_engine {
        SearchEngine::Bing => return bing::extract_links(page),
        SearchEngine::Google => return google::extract_links(page),
    }
}
