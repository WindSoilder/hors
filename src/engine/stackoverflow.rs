use super::Engine;
use serde::Deserialize;

// filter str can help us make less network traffic
// We just need the question_link, and more quota information.
const FILTER_STR: &str = "!)8aEDWeNHfyXN.d";
const VERSION: &str = "2.2";
const API_DOMAIN: &str = "api.stackexchange.com";
const API_KEY: &str = ")y68C9pNW6NnT86cYkKHCQ((";

pub struct StackOverflow {
    api_key: String,
    page_size: u8,
}

impl StackOverflow {
    pub fn new(api_key: String, page_size: u8) -> StackOverflow {
        StackOverflow { api_key, page_size }
    }
}

impl Default for StackOverflow {
    fn default() -> Self {
        StackOverflow::new(API_KEY.to_string(), 10)
    }
}

#[derive(Deserialize, Debug)]
pub struct Questions {
    items: Vec<QuestionItem>,
    quota_max: u16,
    quota_remaining: u16,
}

#[derive(Deserialize, Debug)]
pub struct QuestionItem {
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
            Ok(questions) => Some(questions.items.into_iter().map(|q| q.link).collect()),
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
}
