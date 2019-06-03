use select::document::Document;
use select::predicate::{Class, Name, Predicate};
/// Get google search url.
///
/// # Arguments
///
/// * `query` - The user input query information.
///
/// # Returns
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
/// # Returns
///
/// Links to the relative question, or returns None if we can't find it.
pub fn extract_links(page: &String) -> Option<Vec<String>> {
    let mut links: Vec<String> = Vec::new();
    let doc: Document = Document::from(page.as_str());
    // use child rather than decendent, because in google search engine
    // a node's structure is like this:
    // <r>
    //   <a href="test_link"></a>
    //   <span><a href="not we need"></a></span>
    // </r>
    let target_elements = doc.find(Class("r").child(Name("a")));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let page: String = String::from(
            "
<html>
    <body>
        <div class=\"g\">
            <div class=\"r\">
                <a href=\"https://test_link1\">
                </a>
            </div>
        </div>
        <div class=\"g\">
            <div class=\"r\">
                <a href=\"https://test_link2\">
                </a>
            </div>
        </div>
    </body>
</html>",
        );
        let possible_links: Option<Vec<String>> = extract_links(&page);
        assert_eq!(possible_links.is_some(), true);
        assert_eq!(
            possible_links.unwrap(),
            vec![
                String::from("https://test_link1"),
                String::from("https://test_link2")
            ]
        )
    }

    #[test]
    fn test_extract_links_when_there_are_no_links_available() {
        let page: String = String::from("<html></html>");
        let possible_links: Option<Vec<String>> = extract_links(&page);
        assert_eq!(possible_links.is_none(), true);
    }

    #[test]
    fn test_get_query_url() {
        let result: String = get_query_url(&String::from("how to write unit test"));
        assert_eq!(
            "https://www.google.com/search?q=site:stackoverflow.com%20how to write unit test",
            result
        );
    }
}
