//! This module contains api to get results from stack overflow page.
//! Yeah, our precious lays in stackoverflow.com.

use super::colorize::colorize_code;
use super::crawler::{CrawlerMsg, PageCrawler};
use super::records::AnswerRecordsCache;
use crate::config::{Config, OutputOption};
use crate::error::Result;
use reqwest::{Client, ClientBuilder, Url};
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::collections::HashSet;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub const SPLITTER: &str = "\n^_^ ==================================================== ^_^\n\n";

/// Get answers from given links.
///
/// This function will go through network to find out answers.
///
/// # Examples
///
/// ```rust
/// use hors::{answer, Config, OutputOption};
///
/// # async fn run() {
/// let conf: Config = Config::new(OutputOption::All, 1, false);
/// let links: Vec<String> = vec![
///     String::from("https://stackoverflow.com/questions/7771011/how-to-parse-data-in-json")
/// ];
/// let answers: String = hors::get_answers(&links, conf).await.unwrap();
/// assert!(
///     answers.contains(
///         r#"data = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')"#
///     )
/// );
/// # }
/// ```
///
/// # Returns
///
/// If search answers successfully, it will return the result string which can be
/// print to terminal directly.  Else return an Error.
pub async fn get_answers(links: &[String], conf: Config) -> Result<String> {
    let client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
    get_answers_with_client(links, conf, client).await
}

/// Get answers from given links.
///
/// This function will go through network to find out answers.
///
/// # Examples
///
/// ```rust
/// use hors::{answer, Config, OutputOption};
/// use reqwest::{Client, ClientBuilder};
///
/// # async fn run() {
/// let conf: Config = Config::new(OutputOption::All, 1, false);
/// // please make sure that `cookie_store` should set to `true` in client builder.
/// let mut client: Client = ClientBuilder::new().cookie_store(true).build().unwrap();
/// let links: Vec<String> = vec![
///     String::from("https://stackoverflow.com/questions/7771011/how-to-parse-data-in-json")
/// ];
/// let answers: String = hors::get_answers_with_client(&links, conf, &client).await.unwrap();
/// assert!(
///     answers.contains(
///         r#"data = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')"#
///     )
/// );
/// # }
/// ```
///
/// # Returns
///
/// If search answers successfully, it will return the result string which can be
/// print to terminal directly.  Else return an Error.
pub async fn get_answers_with_client(
    links: &[String],
    conf: Config,
    client: Client,
) -> Result<String> {
    debug!("Try to load cache from local cache file.");
    // load hors internal cache.
    let load_result: Result<AnswerRecordsCache> = AnswerRecordsCache::load();
    let records_cache: AnswerRecordsCache = match load_result {
        Ok(cache) => cache,
        Err(err) => {
            warn!("Can't load cache from local cache file, errmsg {:?}", err);
            AnswerRecordsCache::load_empty()
        }
    };
    debug!("Load cache complete.");

    let results: Result<String> = match conf.option() {
        OutputOption::Links => Ok(answers_links_only(links, conf.numbers() as usize)),
        _ => get_detailed_answer(links, conf, records_cache, client).await,
    };

    results
}

async fn get_detailed_answer(
    links: &[String],
    conf: Config,
    records_cache: AnswerRecordsCache,
    client: Client,
) -> Result<String> {
    let mut results: Vec<String> = Vec::new();

    let (tx, mut rx): (Sender<CrawlerMsg>, Receiver<CrawlerMsg>) = mpsc::channel(10);

    let page_crawler = PageCrawler::new(links.into(), conf, records_cache, client, tx);
    page_crawler.fetch();

    while let Some(page) = rx.recv().await {
        match page {
            CrawlerMsg::Done => break,
            CrawlerMsg::Data(m) => {
                let answer: Option<String> = parse_answer(m.get_page().into(), &conf);
                match answer {
                    Some(content) => results.push(format!("{}\n{}", m.get_title(), content)),
                    None => results.push(format!("Can't get answer from {}", m.get_link())),
                }
            }
        }
    }

    Ok(results.join(SPLITTER))
}

fn parse_answer(page: String, config: &Config) -> Option<String> {
    let doc: Document = Document::from(page.as_str());
    // The question tags may contains useful information about the language topic
    // so syntect can use correct Syntex reference.
    let mut question_tags: Vec<String> = doc
        .find(Class("post-tag"))
        .map(|tag_node| tag_node.text())
        .collect();

    // Ideally, we can take the best SyntaxSet according to user-input code in answers.
    // Like <pre class="lang-rust"><code>println!("hello");</code></pre>.  And we can use rust syntaxset to colorize code.
    // But maybe this property is loaded by `js` dynamiclly, so it's hard to fetch this attribute for now.
    //
    // So we make another way: sort these question tags by the 'Popularity of programming language'.
    // The language is more 'popular', the more possibility to get right syntax set.
    sort_tags(&mut question_tags);

    let appropriate_answer = select_answer(&doc);

    if let Some(answer) = appropriate_answer {
        match *config.option() {
            OutputOption::OnlyCode => {
                return parse_answer_instruction(answer, question_tags, config.colorize());
            }
            OutputOption::All => {
                return parse_answer_detailed(answer, question_tags, config.colorize());
            }
            _ => panic!(
                "parse_answer shoudn't get config with OutputOption::Link.\n
                If you get this message, please fire an issue"
            ),
        }
    }
    None
}

/// Select answer by most voted.
fn select_answer(doc: &Document) -> Option<Node> {
    let mut selected_node: Option<Node> = None;
    let mut selected_voted: i16 = 0;
    let answers = doc.find(Class("answer"));

    for answer in answers {
        // fetch vote count to know which answer is best for users.
        let voted: Node = answer.find(Class("js-vote-count")).next().expect(
            "Can't find vote information :(  If you see this message, please fire an issue.",
        );
        debug!("Voted node infromation {:?}", voted);
        // Hors think that the voted number should less than 32767, so make it i16 type.
        let voted: i16 = voted
            .text()
            .trim()
            .parse()
            .expect("Vote information should be a number :(  If you see this message, please fire an issue.˝");
        if selected_voted < voted {
            selected_voted = voted;
            selected_node = Some(answer);
        }
    }
    selected_node
}

fn parse_answer_instruction(
    answer_node: select::node::Node,
    question_tags: Vec<String>,
    should_colorize: bool,
) -> Option<String> {
    let code_elements: [&str; 2] = ["pre", "code"];
    for code_element in &code_elements {
        if let Some(title) = answer_node.find(Name(*code_element)).next() {
            if should_colorize {
                return Some(colorize_code(title.text(), &question_tags));
            } else {
                return Some(title.text());
            }
        }
    }
    None
}

fn parse_answer_detailed(
    answer_node: select::node::Node,
    question_tags: Vec<String>,
    should_colorize: bool,
) -> Option<String> {
    // stackoverflow may return answer body with `js-post-body` or `post-text` class.
    // so we should class decision first.
    let answer_body = answer_node
        .find(Class("js-post-body"))
        .next()
        .or_else(|| answer_node.find(Class("post-text")).next());
    if let Some(instruction) = answer_body {
        if !should_colorize {
            return Some(instruction.text());
        } else {
            let mut formatted_answer: String = String::new();
            for sub_node in instruction.children() {
                match sub_node.name() {
                    Some("pre") => formatted_answer
                        .push_str(&(colorize_code(sub_node.text(), &question_tags) + "\n")),
                    Some("code") => {
                        formatted_answer.push_str(&colorize_code(sub_node.text(), &question_tags))
                    }
                    Some(_) => formatted_answer.push_str(&(sub_node.text() + "\n\n")),
                    None => continue,
                }
            }
            return Some(formatted_answer);
        }
    }
    None
}

/// Return links from the given stackoverflow links.
///
///
/// # Arguments
///
/// * `links` - stackoverflow links.
///
/// # Returns
/// A list of links with splitter.  Which can directly output by the caller.
fn answers_links_only(links: &[String], restricted_length: usize) -> String {
    let mut results: Vec<String> = Vec::new();
    let mut links_iter = links.iter();
    for _ in 0..restricted_length {
        let next_link = links_iter.next();
        match next_link {
            Some(link) => {
                if !link.contains("question") {
                    continue;
                }
                let url: Url = Url::parse(link)
                    .expect("Parse url failed, if you receive this message, please fire an issue.");

                let answer: String =
                    format!("Title - {}\n{}", extract_question(url.path()), *link,);
                results.push(answer);
            }
            None => break,
        }
    }
    results.join(SPLITTER)
}

/// Extract question content.
///
/// # Examples
///
/// let question: String = extract_question("questions/user_id/the-specific-question");
/// assert_eq!(question, String::from("the specific question"));
fn extract_question(path: &str) -> String {
    // The stack overflow question have the following format
    // https://stackoverflow.com/questions/user_id/the-specific-question
    // we want to extract the question part out.
    let splitted: Vec<&str> = path.split('/').collect();
    splitted[splitted.len() - 1].replace('-', " ")
}

/// Sort question tags inplace.
///
/// It makes some popular *programming languages* tags(like C, C++) to the front of other tags.
///
/// # Examples
///
/// let mut tags: Vec<String> = vec!["json", "rust"];
/// sorted_tags(&mut tags);
/// assert_eq!(tags, vec!["rust", "json"]);
fn sort_tags(tags: &mut Vec<String>) {
    // The list is get from SyntaxSet::load_defaults_newlines().syntaxes();
    // And picks some languages seems more popular.
    let tier_1_tags: HashSet<&str> = [
        "java",
        "javascript",
        "lisp",
        "latex",
        "lua",
        "matlab",
        "ocaml",
        "objective-c++",
        "objective-c",
        "php",
        "pascal",
        "perl",
        "python",
        "r",
        "ruby",
        "rust",
        "scala",
        "c#",
        "c++",
        "c",
        "d",
        "erlang",
        "go",
        "haskell",
    ]
    .iter()
    .cloned()
    .collect();

    tags.sort_by_key(|t| {
        if tier_1_tags.contains(t.as_str()) {
            0
        } else {
            9
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::{Config, OutputOption};

    #[test]
    fn test_answer_links_only() {
        let links: Vec<String> = vec![String::from(
            "https://stackoverflow.com/questions/test/how-to-write-function",
        )];
        let restricted_length: usize = 1;
        let results: String = format!(
            "Title - {}\n{}",
            "how to write function",
            "https://stackoverflow.com/questions/test/how-to-write-function"
        );
        assert_eq!(answers_links_only(&links, restricted_length), results);
    }

    #[test]
    fn test_answer_links_only_when_contains_links_is_not_question() {
        let links: Vec<String> = vec![
            String::from("https://stackoverflow.com/tags/rust"), // this link shouldn't output
            String::from("https://stackoverflow.com/questions/test/how-to-write-function"),
        ];
        let restricted_length: usize = 4;
        let results: String = format!(
            "Title - {}\n{}",
            "how to write function",
            "https://stackoverflow.com/questions/test/how-to-write-function"
        );
        assert_eq!(answers_links_only(&links, restricted_length), results);
    }

    #[test]
    fn test_answer_links_only_when_restricted_size_is_less_than_given_links() {
        let links: Vec<String> = vec![
            String::from("https://stackoverflow.com/questions/test/how-to-write-function"),
            String::from("https://stackoverflow.com/questions/test/best-practise-for-rust"),
        ];
        let restricted_length: usize = 1;
        let results: String = format!(
            "Title - {}\n{}",
            "how to write function",
            "https://stackoverflow.com/questions/test/how-to-write-function"
        );
        assert_eq!(answers_links_only(&links, restricted_length), results);
    }

    #[test]
    fn test_answer_links_only_when_restricted_size_is_large_than_given_links() {
        let links: Vec<String> = vec![
            String::from("https://stackoverflow.com/questions/test/how-to-write-function"),
            String::from("https://stackoverflow.com/questions/test/best-practise-for-rust"),
        ];
        let restricted_length: usize = 1000;
        let results: String = format!(
            "{}\n{}{}{}\n{}",
            "Title - how to write function",
            "https://stackoverflow.com/questions/test/how-to-write-function",
            SPLITTER,
            "Title - best practise for rust",
            "https://stackoverflow.com/questions/test/best-practise-for-rust"
        );
        assert_eq!(answers_links_only(&links, restricted_length), results);
    }

    #[test]
    fn test_parse_answer() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>println!("hello world")</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from(r#"println!("hello world")"#));
        }
    }

    #[test]
    fn test_parse_answer_with_post_text_class() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="post-text">
                        <pre>
                            <code>println!("hello world")</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from(r#"println!("hello world")"#));
        }
    }

    #[test]
    fn test_parse_answer_when_pre_and_code_both_existed() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer <code>goto</code> here </p>
                        <pre>
                            <code>println!("hello world")</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from(r#"println!("hello world")"#));
        }
    }

    #[test]
    fn test_parse_answer_when_no_code_available() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer here </p>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_none(), true);
    }

    #[test]
    fn test_parse_answer_when_only_code_existed() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer <code>goto</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("goto"));
        }
    }

    #[test]
    fn test_parse_answer_detailed() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer <code>goto</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("answer goto here"));
        }
    }

    #[test]
    fn test_parse_answer_detailed_when_no_code_available() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer goto here</p>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("answer goto here"));
        }
    }

    #[test]
    fn test_parse_answer_detailed_when_only_pre_code() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            print('go go go')
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("print('go go go')"));
        }
    }

    #[test]
    fn test_parse_answer_when_two_answers_available() {
        let page: String = String::from(
            r#"
        <html>
            <body>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <p>answer <code>lower</code> here </p>
                    </div>
                </div>
                <div class="answer">
                    <div class="js-vote-count">9000</div>
                    <div class="js-post-body">
                        <p>answer <code>higher</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("answer higher here"));
        }
    }

    #[test]
    fn test_parse_answer_colorized() {
        // to testing answer colorized, we just want to make sure that
        // the result has different length.
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag">python</a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let un_colorized_answer: String = parse_answer(page, &conf).unwrap();
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, true);
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag">python</a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let colorized_answer: String = parse_answer(page, &conf).unwrap();
        assert_ne!(un_colorized_answer.trim(), colorized_answer.trim());
        assert!(un_colorized_answer.trim().len() < colorized_answer.trim().len());
    }

    #[test]
    fn test_parse_answer_detailed_colorized() {
        // to testing answer colorized, we just want to make sure that
        // the result has different length.
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag">python</a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let un_colorized_answer: String = parse_answer(page, &conf).unwrap();
        let conf: Config = Config::new(OutputOption::All, 1, true);
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag">python</a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let colorized_answer: String = parse_answer(page, &conf).unwrap();
        assert_ne!(un_colorized_answer.trim(), colorized_answer.trim());
        assert!(un_colorized_answer.trim().len() < colorized_answer.trim().len());
    }

    #[test]
    fn test_parse_answer_colorized_when_no_tags_available() {
        // when no tags information lays in the page, it should work too.
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag"></a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let un_colorized_answer: String = parse_answer(page, &conf).unwrap();
        let conf: Config = Config::new(OutputOption::All, 1, true);
        let page: String = String::from(
            r#"
        <html>
            <body>
                <a class="post-tag"></a>
                <div class="answer">
                    <div class="js-vote-count">130</div>
                    <div class="js-post-body">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        "#,
        );
        let colorized_answer: String = parse_answer(page, &conf).unwrap();
        assert_ne!(un_colorized_answer.trim(), colorized_answer.trim());
        assert!(un_colorized_answer.trim().len() < colorized_answer.trim().len());
    }

    #[test]
    fn test_parse_answer_when_no_answers_available() {
        let page: String = String::from("");
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_none(), true);
    }

    #[test]
    fn test_parse_answer_detailed_when_no_answers_available() {
        let page: String = String::from("");
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);
        assert_eq!(answer.is_none(), true);
    }

    #[test]
    fn test_extract_question() {
        let question: String = extract_question("questions/user_id/the-specific-question");
        assert_eq!(question, String::from("the specific question"));
    }

    #[test]
    fn test_extract_question_when_question_contains_one_word() {
        let question: String = extract_question("questions/user_id/question");
        assert_eq!(question, String::from("question"));
    }

    #[test]
    fn test_sort_tags_when_no_tags() {
        let mut tags = vec![];
        sort_tags(&mut tags);
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_sort_tags_contains_all_pupular_lang_tags() {
        let mut tags = vec!["rust".to_string(), "python".to_string()];
        sort_tags(&mut tags);
        assert_eq!(tags, vec!["rust".to_string(), "python".to_string()]);
    }

    #[test]
    fn test_sort_tags_contains_all_unpopular_lang_tags() {
        let mut tags = vec!["json".to_string(), "xml".to_string()];
        sort_tags(&mut tags);
        assert_eq!(tags, vec!["json".to_string(), "xml".to_string()]);
    }

    #[test]
    fn test_sort_tags_contains_both_popular_and_unpupular_lang_tags() {
        let mut tags = vec![
            "json".to_string(),
            "rust".to_string(),
            "xml".to_string(),
            "java".to_string(),
        ];
        sort_tags(&mut tags);
        assert_eq!(
            tags,
            vec![
                "rust".to_string(),
                "java".to_string(),
                "json".to_string(),
                "xml".to_string()
            ]
        );
    }
}
