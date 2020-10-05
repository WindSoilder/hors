use super::Engine;
use crate::search_config::SEARCH_CONFIG;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use url::Url;

pub struct Google;

impl Engine for Google {
    fn get_query_url(&self, query: &str, use_https: bool) -> String {
        if use_https {
            format!(
                "https://{}/search?q=site:stackoverflow.com%20{}",
                SEARCH_CONFIG.get_google_domain(),
                query
            )
        } else {
            format!(
                "http://{}/search?q=site:stackoverflow.com%20{}",
                SEARCH_CONFIG.get_google_domain(),
                query
            )
        }
    }

    fn extract_links(&self, page: &str) -> Option<Vec<String>> {
        let doc: Document = Document::from(page);

        // new style target elements, which have rc class.
        let target_elements = doc.find(Class("rc").descendant(Name("a")));
        let mut links: Vec<String> = target_elements
            .filter_map(|node| node.attr("href"))
            .filter_map(|link| {
                if let Ok(url) = Url::parse(link) {
                    if let Some(host) = url.host_str() {
                        if host.contains("stackoverflow.com") {
                            return Some(String::from(link));
                        }
                    }
                }
                None
            })
            .collect();
        if links.is_empty() {
            // old style target elements.
            // use child rather than descendant, because in google search engine
            // a node's structure is like this:
            // <div class="rc">
            //   <a href="test_link"></a>
            //   <span><a href="not we need"></a></span>
            // </div>
            links = doc
                .find(Class("r").descendant(Name("a")))
                .filter_map(|node| node.attr("href"))
                .map(String::from)
                .collect()
        }

        debug!("Links extract from google: {:?}", links);
        if links.is_empty() {
            return None;
        }
        Some(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let engine = Google;
        let page: String = String::from(
            r#"
<html>
    <body>
        <div class="g">
            <div class="r">
                <a href="https://test_link1">
                </a>
            </div>
        </div>
        <div class="g">
            <div class="r">
                <a href="https://test_link2">
                </a>
            </div>
        </div>
    </body>
</html>"#,
        );
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
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
        let engine = Google;
        let page: String = String::from("<html></html>");
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
        assert_eq!(possible_links.is_none(), true);
    }

    #[test]
    fn test_get_query_url() {
        let engine = Google;
        let result: String = engine.get_query_url(&String::from("how to write unit test"), true);
        assert_eq!(
            "https://www.google.com/search?q=site:stackoverflow.com%20how to write unit test",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let engine = Google;
        let result: String = engine.get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://www.google.com/search?q=site:stackoverflow.com%20how to write unit test",
            result
        );
    }

    #[test]
    fn test_extract_links_new_style() {
        let engine = Google;
        let page: String = String::from(
            r#"
<html>
    <body>
        <div class="g">
            <div class="rc">
                <div class="tmp">
                    <a href="https://test_link1?site=stackoverflow.com">
                    </a>
                </div>
            </div>
        </div>
        <div class="g">
            <div class="rc">
                <div class="tmp">
                    <a href="https://stackoverflow.com/aaa">
                    </a>
                </div>
            </div>
        </div>
    </body>
</html>"#,
        );
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
        assert_eq!(possible_links.is_some(), true);
        assert_eq!(
            possible_links.unwrap(),
            vec![String::from("https://stackoverflow.com/aaa")]
        )
    }
}
