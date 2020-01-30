use select::document::Document;
use select::predicate::{Class, Name, Predicate};

/// Get bing search url.
///
/// # Arguments
///
/// * `query` - The user input query information.
/// * `use_https` - Return query url which is https scheme or http scheme.
///
/// # Returns
///
/// Return the query url, which can be fired with HTTP GET request.
pub fn get_query_url(query: &str, use_https: bool) -> String {
    if use_https {
        format!(
            "https://www.bing.com/search?q=site:stackoverflow.com%20{}",
            query
        )
    } else {
        format!(
            "http://www.bing.com/search?q=site:stackoverflow.com%20{}",
            query
        )
    }
}

/// Extract links from given page.
///
/// # Arguments
///
/// * `page` - the bing search result page.
///
/// # Returns
///
/// Links to the relative question, or returns None if we can't find it.
pub fn extract_links(page: &str) -> Option<Vec<String>> {
    let doc: Document = Document::from(page);
    let target_elements = doc.find(Class("b_algo").descendant(Name("h2")).descendant(Name("a")));
    let links: Vec<String> = target_elements
        .filter_map(|node| node.attr("href"))
        .filter(|link| link.contains("stackoverflow.com"))
        .map(|link| String::from(link))
        .collect();

    debug!("Links extract from bing: {:?}", links);
    if links.is_empty() {
        return None;
    }
    Some(links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let page: String = String::from(
            r#"
<html>
    <body>
        <li class="b_algo">
            <h2><a target="_blank" href="https://test_link_invalid.com"></a></h2>
        </li>
        <li class="b_algo">
            <h2><a target="_blank" href="https://stackoverflow.com"></a></h2>
        </li>
        <li class="b_algo">
            <h2><a target="_blank" href="https://stackoverflow.com/aa"></a></h2>
        </li>
    </body>
</html>"#,
        );
        let possible_links: Option<Vec<String>> = extract_links(&page);
        assert_eq!(possible_links.is_some(), true);
        assert_eq!(
            possible_links.unwrap(),
            vec![
                String::from("https://stackoverflow.com"),
                String::from("https://stackoverflow.com/aa")
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
        let result: String = get_query_url(&String::from("how to write unit test"), true);
        assert_eq!(
            "https://www.bing.com/search?q=site:stackoverflow.com%20how to write unit test",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let result: String = get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://www.bing.com/search?q=site:stackoverflow.com%20how to write unit test",
            result
        );
    }
}
