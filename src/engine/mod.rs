mod bing;
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
///
/// # Return value
///
/// If search links successfully, it will return a Vector of String, which indicate
/// relative links to got answer.  Else return an Error.
pub fn search_links(query: &String, engine: &String) -> Result<Vec<String>> {
    let fetch_url: String = get_query_url(query, engine);
    let page: String = fetch(&fetch_url)?;
    let extract_results = extract_links(&page, engine);
    match extract_results {
        Some(links) => return Ok(links),
        None => {
            return Err(HorsError::from_parse("Can't find search result..."));
        }
    }
}

fn get_query_url(query: &String, engine: &String) -> String {
    return bing::get_query_url(query);
}

/// Fetch actual page according to given query.
///
/// # Arguments
///
/// * `query` - The user input query String.
///
/// # Return value
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
/// * `page` - the bing search result page, which is mainly got by `fetch` function
///
/// # Return value
///
/// Links to the relative question, or returns None if we can't find it.
fn extract_links(page: &String, engine: &String) -> Option<Vec<String>> {
    return bing::extract_links(page);
}
