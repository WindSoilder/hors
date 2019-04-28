extern crate reqwest;
extern crate select;

use crate::error::HorError;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::error::Error;

/// Search under the `bing` search engine.
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
pub fn search(query: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let page: String = fetch(query)?;
    let extract_results = extract_links(&page);
    match extract_results {
        Some(links) => return Ok(links),
        None => {
            return Result::Err(Box::new(HorError::from_parse(
                "Can't find search result...",
            )));
        }
    }
}

/// fetch actual page according to given query.
///
/// # Arguments
///
/// * `query` - The user input query String.
///
/// # Return value
///
/// If get search result page successfully, it will return the content of page,
/// or returns error.
fn fetch(query: &String) -> Result<String, Box<dyn Error>> {
    let url: String = format!(
        "https://www.bing.com/search?q=site:stackoverflow.com%20{}",
        query
    );
    let client = reqwest::Client::new();
    let mut res = client
        .get(url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:66.0) Gecko/20100101 Firefox/66.0",
        )
        .send()?;
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
fn extract_links(page: &String) -> Option<Vec<String>> {
    let mut links: Vec<String> = Vec::new();
    let doc: Document = Document::from(page.as_str());
    let target_elements = doc.find(Class("b_algo").descendant(Name("h2")).descendant(Name("a")));
    for node in target_elements {
        if let Some(link) = node.attr("href") {
            links.push(String::from(link));
        }
    }

    if links.len() == 0 {
        return None;
    }
    return Some(links);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_links() {}

    #[test]
    fn test_extract_links_when_there_are_no_links_available() {}
}
