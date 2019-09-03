use select::document::Document;
use select::predicate::Class;
use url::form_urlencoded;

/// Get DuckDuckgo search url.
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
    // For more information about query url, the information here is useful:
    // https://stackoverflow.com/questions/37012469/duckduckgo-api-getting-search-results
    if use_https {
        format!(
            "https://duckduckgo.com/html?q=site:stackoverflow.com%20{}&t=hj&ia=web",
            query
        )
    } else {
        format!(
            "http://duckduckgo.com/html?q=site:stackoverflow.com%20{}&t=hj&ia=web",
            query
        )
    }
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
pub fn extract_links(page: &str) -> Option<Vec<String>> {
    let doc: Document = Document::from(page);
    let target_elements = doc.find(Class("result__a"));
    let links: Vec<String> = target_elements
        .filter_map(|node| node.attr("href"))
        .filter_map(|link| {
            if link.starts_with("/l/?") {
                // DuckDuckGo redirect link
                // e.g. /l/?kh=-1&uddg=https%3A%2F%2Fdoc.rust%2Dlang.org%2Fstd%2Fprimitive.str.html
                debug!("Extracting URL from redirect link {:?}", link);
                let query = &link[4..]; // trim "/l/?"
                form_urlencoded::parse(query.as_bytes())
                    .find(|(k, _)| k == "uddg")
                    .map(|(_, v)| v.into_owned())
            } else {
                Some(String::from(link))
            }
        })
        .collect();

    debug!("Links extrace from duckduckgo: {:?}", links);
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
            "
<html>
    <body>
        <div class=\"result__body\">
            <a class=\"result__a\" href=\"https://test_link1\"></a>
        </div>
        div class=\"result__body\">
            <a class=\"result__a\" href=\"https://test_link2\"></a>
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
        let result: String = get_query_url(&String::from("how to write unit test"), true);
        assert_eq!(
            "https://duckduckgo.com/html?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let result: String = get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://duckduckgo.com/html?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }
}
