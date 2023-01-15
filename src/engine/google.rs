use super::Engine;
use crate::search_config::SEARCH_CONFIG;
use regex::Regex;
use std::collections::HashSet;

pub struct Google;

impl Engine for Google {
    fn get_query_url(&self, query: &str, use_https: bool) -> String {
        if use_https {
            format!(
                "https://{}/search?q=site:stackoverflow.com%20{}&hl=en",
                SEARCH_CONFIG.get_google_domain(),
                query
            )
        } else {
            format!(
                "http://{}/search?q=site:stackoverflow.com%20{}&hl=en",
                SEARCH_CONFIG.get_google_domain(),
                query
            )
        }
    }

    fn extract_links(&self, page: &str) -> Option<Vec<String>> {
        let link_pattern =
            Regex::new(r#"https?://*stackoverflow.com/questions/[0-9]*/[a-z0-9-]*"#).unwrap();

        let mut link_set = HashSet::with_capacity(10);
        for link in link_pattern.captures_iter(page) {
            // the `link.get(0)` always return a entire matched string.
            let link_str = link.get(0).unwrap().as_str();
            if !link_str.contains("/url?") {
                link_set.insert(link_str.to_string());
            }
        }

        debug!("Links extract from google: {:?}", link_set);
        if link_set.is_empty() {
            None
        } else {
            Some(link_set.into_iter().collect())
        }
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
                <a href="https://stackoverflow.com/questions/12/asdf">
                </a>
            </div>
        </div>
        <div class="g">
            <div class="r">
                <a href="https://stackoverflow.com/questions/34/dfs">
                </a>
            </div>
        </div>
    </body>
</html>"#,
        );
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
        assert_eq!(possible_links.is_some(), true);
        assert_eq!(
            possible_links
                .unwrap()
                .into_iter()
                .collect::<HashSet<String>>(),
            vec![
                String::from("https://stackoverflow.com/questions/34/dfs"),
                String::from("https://stackoverflow.com/questions/12/asdf")
            ]
            .into_iter()
            .collect::<HashSet<String>>()
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
            "https://www.google.com/search?q=site:stackoverflow.com%20how to write unit test&hl=en",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let engine = Google;
        let result: String = engine.get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://www.google.com/search?q=site:stackoverflow.com%20how to write unit test&hl=en",
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
                    <a href="https://stackoverflow.com/questions/12/asdf">
                    </a>
                </div>
            </div>
        </div>
        <div class="g">
            <div class="rc">
                <div class="tmp">
                    <a href="https://stackoverflow.com/questions/34/dfs">
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
            possible_links
                .unwrap()
                .into_iter()
                .collect::<HashSet<String>>(),
            vec![
                String::from("https://stackoverflow.com/questions/12/asdf"),
                String::from("https://stackoverflow.com/questions/34/dfs"),
            ]
            .into_iter()
            .collect::<HashSet<String>>()
        )
    }
}
