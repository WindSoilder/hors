mod bing;
mod duckduckgo;
mod google;

use crate::config::SearchEngine;
use crate::error::{HorsError, Result};
use crate::utils::random_agent;
use reqwest::{Client, RequestBuilder};

/// Search result links under the given search engine.
///
/// This function will go through network to find out useful links in bing.
///
/// # Arguments
///
/// * `query` - The user input query String.
/// * `search_engine` - indicate which search engine we use to search result links.
/// * `client` - reqwest::Client object, please ensure that it's build with cookie_store(true) option.
///
/// # Returns
///
/// If search links successfully, it will return a Vector of String, which indicate
/// relative links to got answer.  Else return an Error.
pub fn search_links(
    query: &str,
    search_engine: SearchEngine,
    client: &Client,
) -> Result<Vec<String>> {
    let https_opts: Vec<bool> = vec![true, false];
    for opt in https_opts {
        let fetch_url: String = get_query_url(query, &search_engine, opt);
        let page: String = fetch(&fetch_url, client)?;
        let extract_results = extract_links(&page, &search_engine);
        if let Some(links) = extract_results {
            return Ok(links);
        }
    }
    Err(HorsError::from_parse("Can't find search result..."))
}

fn get_query_url(query: &str, search_engine: &SearchEngine, use_https: bool) -> String {
    match search_engine {
        SearchEngine::Bing => bing::get_query_url(query, use_https),
        SearchEngine::Google => google::get_query_url(query, use_https),
        SearchEngine::DuckDuckGo => duckduckgo::get_query_url(query, use_https),
    }
}

/// Fetch actual page according to given url.
///
/// # Arguments
///
/// * `search_url` - The url which should lead to search result page.
/// * `client` - An instance of `request::Client` object which can use to fire http request,
///              please ensure that it's build with cookie_store(true) option.
///
/// # Returns
///
/// If get search result page successfully, it will return the content of page,
/// or returns error.
fn fetch(search_url: &str, client: &Client) -> Result<String> {
    let request: RequestBuilder = client
        .get(search_url)
        .header(reqwest::header::USER_AGENT, random_agent());
    debug!("Request to bing information: {:?}", request);
    let mut res = request.send()?;
    let page: String = res.text()?;
    Ok(page)
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
fn extract_links(page: &str, search_engine: &SearchEngine) -> Option<Vec<String>> {
    match search_engine {
        SearchEngine::Bing => bing::extract_links(page),
        SearchEngine::Google => google::extract_links(page),
        SearchEngine::DuckDuckGo => duckduckgo::extract_links(page),
    }
}
