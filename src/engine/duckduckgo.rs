use select::document::Document;
use select::predicate::Class;
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
pub fn get_query_url(query: &String, use_https: bool) -> String {
    if use_https {
        format!(
            "https://duckduckgo.com/?q=site:stackoverflow.com%20{}&t=hj&ia=web",
            query
        )
    } else {
        format!(
            "http://duckduckgo.com/?q=site:stackoverflow.com%20{}&t=hj&ia=web",
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
pub fn extract_links(page: &String) -> Option<Vec<String>> {
    let doc: Document = Document::from(page.as_str());
    println!("{}", page);
    let target_elements = doc.find(Class("result__a"));
    let mut links: Vec<String> = Vec::new();

    for node in target_elements {
        if let Some(link) = node.attr("href") {
            links.push(String::from(link));
        }
    }

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
            "https://duckduckgo.com/?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let result: String = get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://duckduckgo.com/?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }
}
