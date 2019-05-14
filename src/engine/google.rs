use select::document::Document;
use select::predicate::{Class, Name, Predicate};

/// Get google search url.
///
/// # Arguments
///
/// * `query` - The user input query information.
///
/// # Return value
///
/// Return the query url, which can be fired with HTTP GET request.
pub fn get_query_url(query: &String) -> String {
    return format!(
        "https://www.google.com/search?q=site:stackoverflow.com%20{}",
        query
    );
}

/// Extract links from given page.
///
/// # Arguments
///
/// * `page` - the google search result page.
///
/// # Return value
///
/// Links to the relative question, or returns None if we can't find it.
pub fn extract_links(page: &String) -> Option<Vec<String>> {
    let mut links: Vec<String> = Vec::new();
    let doc: Document = Document::from(page.as_str());
    let target_elements = doc.find(Class("r").descendant(Name("a")));
    for node in target_elements {
        if let Some(link) = node.attr("href") {
            links.push(String::from(link));
        }
    }

    debug!("Links extract from google: {:?}", links);
    if links.len() == 0 {
        return None;
    }
    return Some(links);
}
