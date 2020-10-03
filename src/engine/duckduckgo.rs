use super::Engine;
use crate::search_config::SEARCH_CONFIG;
use select::document::Document;
use select::predicate::Class;
use url::form_urlencoded;

pub struct DuckDuckGo;

impl Engine for DuckDuckGo {
    fn get_query_url(&self, query: &str, use_https: bool) -> String {
        // For more information about query url, the information here is useful:
        // https://stackoverflow.com/questions/37012469/duckduckgo-api-getting-search-results
        if use_https {
            format!(
                "https://{}/html?q=site:stackoverflow.com%20{}&t=hj&ia=web",
                SEARCH_CONFIG.get_ddg_domain(),
                query
            )
        } else {
            format!(
                "http://{}/html?q=site:stackoverflow.com%20{}&t=hj&ia=web",
                SEARCH_CONFIG.get_ddg_domain(),
                query
            )
        }
    }

    fn extract_links(&self, page: &str) -> Option<Vec<String>> {
        let doc: Document = Document::from(page);
        let target_elements = doc.find(Class("result__a"));
        let links: Vec<String> = target_elements
            .filter_map(|node| node.attr("href"))
            .filter_map(|link| {
                // The link may hide in uddg attribute.
                // e.g: /l/?kh=-1&uddg=https%3A%2F%2Fdoc.rust%2Dlang.org%2Fstd%2Fprimitive.str.html
                // So try to find uddg parameter first.
                let redirect_link = form_urlencoded::parse(link.as_bytes())
                    .find(|(k, _)| k == "uddg")
                    .map(|(_, v)| v.into_owned());
                // If we can't find redirect link in uddg, just return link.
                redirect_link.or(Some(String::from(link)))
            })
            .collect();

        debug!("Links extrace from duckduckgo: {:?}", links);
        if links.is_empty() {
            warn!(
                "Can't get search result from duckduckgo, source page\n{}",
                page
            );
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
        let engine = DuckDuckGo;
        let page: String = String::from(
            r#"
<html>
    <body>
        <div class="result__body">
            <a class="result__a" href="https://test_link1"></a>
        </div>
        div class="result__body">
            <a class="result__a" href="https://test_link2"></a>
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
        let engine = DuckDuckGo;
        let page: String = String::from("<html></html>");
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
        assert_eq!(possible_links.is_none(), true);
    }

    #[test]
    fn test_extract_links_when_ddg_provide_redirect_links() {
        let page: String = String::from(
            r#"
<html>
    <body>
        <div class="result__body">
            <a class="result__a" href="/l/?kh=-1&uddg=https%3A%2F%2Ftest_link1"></a>
        </div>
        div class="result__body">
            <a class="result__a" href="/l/?kh=-1&uddg=https%3A%2F%2Ftest_link2"></a>
        </div>
    </body>
</html>"#,
        );
        let engine = DuckDuckGo;
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
    fn test_extract_links_when_ddg_provide_redirect_links_but_no_uddg_attributes() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="result__body">
                    <a class="result__a" href="/l/?kh=-1"></a>
                </div>
                div class="result__body">
                    <a class="result__a" href="/l/?kh=-1"></a>
                </div>
            </body>
        </html>"#,
        );
        let engine = DuckDuckGo;
        let possible_links: Option<Vec<String>> = engine.extract_links(&page);
        assert_eq!(possible_links.is_some(), true);
    }

    #[test]
    fn test_get_query_url() {
        let engine = DuckDuckGo;
        let result: String = engine.get_query_url(&String::from("how to write unit test"), true);
        assert_eq!(
            "https://duckduckgo.com/html?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let engine = DuckDuckGo;
        let result: String = engine.get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            "http://duckduckgo.com/html?q=site:stackoverflow.com%20how to write unit test&t=hj&ia=web",
            result
        );
    }
}
