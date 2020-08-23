use super::Engine;
use serde::{Deserialize, Serialize};

// filter str can help us make less network traffic
// We just need the question_link.
// const filter_str: &str = "!C(oADSp5jBbfLaHwU";
const filter_str: &str = "!)8aEDWeNHa1LAod";
const version: &str = "2.2";
const domain_name: &str = "api.stackexchange.com";

pub struct StackOverflow {
    api_key: String,
    page_size: u8,
}

impl StackOverflow {
    pub fn new() -> StackOverflow {
        StackOverflow {
            api_key: ")y68C9pNW6NnT86cYkKHCQ((".to_string(),
            page_size: 10,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Questions {
    items: Vec<QuestionItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QuestionItem {
    link: String,
}

impl Engine for StackOverflow {
    fn get_query_url(&self, query: &str, use_https: bool) -> String {
        let scheme = if use_https { "https" } else { "http" };
        format!(
                "{}://{}/{}/search/advanced?key={}&pagesize=10&site=stackoverflow&order=desc&sort=relevance&q={}&filter={}",
                scheme, domain_name, version, self.api_key, query, filter_str
        )
    }

    fn extract_links(&self, pages: &str) -> Option<Vec<String>> {
        let questions = serde_json::from_str::<Questions>(pages).unwrap().items;
        Some(questions.into_iter().map(|x| x.link).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_query_url() {
    //     let engine = StackOverflow::new("aaa".to_string());
    //     let result: String = engine.get_query_url(&String::from("how to write unit test"), true);
    //     assert_eq!(
    //         result,
    //         format!("https://api.stackexchange.com/2.2/search/advanced?\
    //         key=aaa&pagesite=stackoverflow&order=desc&sort=relevance&q=how to write unit test&filter={}", "!C(oADSp5jBbfLaHwU")
    //     );
    // }

    // #[test]
    // fn test_get_query_url_with_https_option_disabled() {
    //     let engine = StackOverflow::new("aaa".to_string());
    //     let result: String = engine.get_query_url(&String::from("how to write unit test"), false);
    //     assert_eq!(
    //         result,
    //         format!("http://api.stackexchange.com/2.2/search/advanced?\
    //         key=aaa&site=stackoverflow&order=desc&sort=relevance&q=how to write unit test&filter={}", "!C(oADSp5jBbfLaHwU")
    //     )
    // }
}
