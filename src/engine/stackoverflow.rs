use super::Engine;
use serde::Deserialize;

// filter str can help us make less network traffic
// We just need the question_link, and more quota information.
const FILTER_STR: &str = "!)8aEDWeNHfyXN.d";

const VERSION: &str = "2.2";
const API_DOMAIN: &str = "api.stackexchange.com";
const API_KEY: &str = ")y68C9pNW6NnT86cYkKHCQ((";

/// StackOverflow search engine.
/// The search engine use stackexchange API to make search.
/// Reach https://api.stackexchange.com/docs/advanced-search to see more usage details.
pub struct StackOverflow {
    /// Request key for stackexchange.
    api_key: String,
    /// The length of questions we need to fetch in a query.
    page_size: u8,
}

impl StackOverflow {
    pub fn new(api_key: String, page_size: u8) -> StackOverflow {
        StackOverflow { api_key, page_size }
    }
}

impl Default for StackOverflow {
    fn default() -> Self {
        // By default we only need to search 10 records per query.
        StackOverflow::new(API_KEY.to_string(), 10)
    }
}

#[derive(Deserialize, Debug)]
struct Questions {
    items: Vec<QuestionItem>,
    quota_max: u16,
    quota_remaining: u16,
}

impl Questions {
    pub fn into_iter(self) -> std::vec::IntoIter<QuestionItem> {
        self.items.into_iter()
    }
}

#[derive(Deserialize, Debug)]
struct QuestionItem {
    link: String,
}

impl Engine for StackOverflow {
    fn get_query_url(&self, query: &str, use_https: bool) -> String {
        let scheme = if use_https { "https" } else { "http" };
        format!(
                "{}://{}/{}/search/advanced?key={}&pagesize={}&site=stackoverflow&order=desc&sort=relevance&q={}&filter={}",
                scheme, API_DOMAIN, VERSION, self.api_key, self.page_size, query, FILTER_STR
        )
    }

    fn extract_links(&self, pages: &str) -> Option<Vec<String>> {
        let deser_result = serde_json::from_str::<Questions>(pages);
        match deser_result {
            Err(e) => {
                warn!("Deserialize json response failed: {}", e);
                None
            }
            Ok(questions) => Some(questions.into_iter().map(|q| q.link).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query_url() {
        let engine = StackOverflow::new("aaa".to_string(), 10);
        let result: String = engine.get_query_url(&String::from("how to write unit test"), true);
        assert_eq!(
            result,
            format!("https://api.stackexchange.com/2.2/search/advanced?\
            key=aaa&pagesize=10&site=stackoverflow&order=desc&sort=relevance&q=how to write unit test&filter={}", "!)8aEDWeNHfyXN.d")
        );
    }

    #[test]
    fn test_get_query_url_with_https_option_disabled() {
        let engine = StackOverflow::new("aaa".to_string(), 10);
        let result: String = engine.get_query_url(&String::from("how to write unit test"), false);
        assert_eq!(
            result,
            format!("http://api.stackexchange.com/2.2/search/advanced?\
            key=aaa&pagesize=10&site=stackoverflow&order=desc&sort=relevance&q=how to write unit test&filter={}", "!)8aEDWeNHfyXN.d")
        )
    }

    #[test]
    fn test_extract_links() {
        let engine = StackOverflow::new("aaa".to_string(), 10);
        let result = engine.extract_links(
            r#"
            {
                "items": [{"link": "http://aaa.com/"}, {"link": "http://aaa.com/bb"}],
                "quota_max": 10,
                "quota_remaining": 9
        }"#,
        );
        assert_eq!(result.is_some(), true);
        assert_eq!(
            result.unwrap(),
            vec![
                String::from("http://aaa.com/"),
                String::from("http://aaa.com/bb")
            ]
        )
    }

    #[test]
    fn test_extract_links_with_wrong_format() {
        let engine = StackOverflow::new("aaa".to_string(), 10);
        let possible_links: Option<Vec<String>> = engine.extract_links(
            r#"
            {
                "items": [],
                "quota_remaining": 9
            }"#,
        );
        assert_eq!(possible_links.is_none(), true);
    }

    #[test]
    fn test_extract_links_with_no_items() {
        let engine = StackOverflow::new("aaa".to_string(), 10);
        let possible_links: Option<Vec<String>> = engine.extract_links(
            r#"
            {
                "items": [],
                "quota_max": 10,
                "quota_remaining": 9
            }"#,
        );
        let expected: Vec<String> = vec![];
        assert_eq!(possible_links.is_some(), true);
        assert_eq!(possible_links.unwrap(), expected);
    }
}
