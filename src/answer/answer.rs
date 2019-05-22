//! This module contains api to get results from stack overflow page
use crate::config::{Config, OutputOption};
use crate::error::Result;
use crate::utils::random_agent;
use reqwest::{Response, Url};
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

const SPLITTER: &str = "\n^_^ ==================================================== ^_^\n\n";

/// Get answers from given links.
///
/// This function will go through network to find out answers.
///
/// # Arguments
///
/// * `links` - the links where answer existed.
/// * `conf` - contains information about get_answer options.
///
/// # Return value
///
/// If search answers successfully, it will return the result string which can be
/// print to terminal directly.  Else return an Error.
pub fn get_answers(links: &Vec<String>, conf: Config) -> Result<String> {
    match conf.option() {
        OutputOption::Links => return Ok(answers_links_only(links, conf.numbers() as usize)),
        _ => return get_detailed_answer(links, conf),
    }
}

fn get_detailed_answer(links: &Vec<String>, conf: Config) -> Result<String> {
    let mut results: Vec<String> = Vec::new();
    let user_agent: &str = random_agent();
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    let mut links_iter = links.iter();

    for _ in 0..conf.numbers() {
        let next_link = links_iter.next();
        match next_link {
            Some(link) => {
                // the given links may contains the url doesn't contains `question`
                // tag, so it's not a question, and just deal with nothing to it.
                if !link.contains("question") {
                    continue;
                }
                /*
                TODO:
                Firstly try to get link from cache, when we can get answer from cache
                if user want to colorize the result, we can pass config to colorized.
                then we can just retrive results from that cache
                but if we can't find result from cache, then we have to get answer from network.
                And save the results from network to inner struct AnswerRecords.
                */
                let mut resp: Response = client
                    .get(link)
                    .header(reqwest::header::USER_AGENT, user_agent)
                    .send()?;
                debug!("Response status from stackoverflow: {:?}", resp);
                let page: String = resp.text()?;
                let title: String = format!("- Answer from {}", link);
                let answer: Option<String> = parse_answer(page, &conf);
                match answer {
                    Some(content) => results.push(format!("{}\n{}", title, content)),
                    None => results.push(format!("Can't get answer from {}", link)),
                }
            }
            None => break,
        }
    }
    return Ok(results.join(SPLITTER));
}

fn parse_answer(page: String, config: &Config) -> Option<String> {
    let doc: Document = Document::from(page.as_str());
    // The question tags may contains useful information about the language topic
    // so syntect can use correct Syntex reference.
    let mut question_tags: Vec<String> = vec![];
    let tags = doc.find(Class("post-tag"));
    for tag in tags {
        question_tags.push(tag.text());
    }

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
    return None;
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
        if voted > selected_voted {
            selected_voted = voted;
            selected_node = Some(answer);
        }
    }
    return selected_node;
}

fn parse_answer_instruction(
    answer_node: select::node::Node,
    question_tags: Vec<String>,
    should_colorize: bool,
) -> Option<String> {
    let code_elements: [&str; 2] = ["pre", "code"];
    for code_element in code_elements.iter() {
        if let Some(title) = answer_node.find(Name(*code_element)).next() {
            if should_colorize {
                return Some(colorized_code(title.text(), &question_tags));
            } else {
                return Some(title.text());
            }
        }
    }
    return None;
}

fn parse_answer_detailed(
    answer_node: select::node::Node,
    question_tags: Vec<String>,
    should_colorize: bool,
) -> Option<String> {
    if let Some(instruction) = answer_node.find(Class("post-text")).next() {
        if should_colorize == false {
            return Some(instruction.text());
        } else {
            let mut formatted_answer: String = String::new();
            for sub_node in instruction.children() {
                match sub_node.name() {
                    Some("pre") => formatted_answer
                        .push_str(&(colorized_code(sub_node.text(), &question_tags) + "\n")),
                    Some("code") => {
                        formatted_answer.push_str(&colorized_code(sub_node.text(), &question_tags))
                    }
                    Some(_) => formatted_answer.push_str(&(sub_node.text() + "\n\n")),
                    None => continue,
                }
            }
            return Some(formatted_answer);
        }
    }
    return None;
}

/// make code block colorized.
///
/// Note that this function should only accept code block.
fn colorized_code(code: String, possible_tags: &Vec<String>) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts: ThemeSet = ThemeSet::load_defaults();
    let syntax: &SyntaxReference = guess_syntax(&possible_tags, &ss);
    let mut h = HighlightLines::new(&syntax, &ts.themes["base16-eighties.dark"]);
    let mut colorized: String = String::new();

    for line in LinesWithEndings::from(code.as_str()) {
        let escaped = as_24_bit_terminal_escaped(&h.highlight(line, &ss), false);
        colorized = colorized + escaped.as_str();
    }
    return colorized;
}

fn guess_syntax<'a>(possible_tags: &Vec<String>, ss: &'a SyntaxSet) -> &'a SyntaxReference {
    for tag in possible_tags {
        let syntax = ss.find_syntax_by_token(tag.as_str());
        if let Some(result) = syntax {
            return result;
        }
    }
    return ss.find_syntax_plain_text();
}

/// Return links from the given stackoverflow links
///
///
/// # Arguments
///
/// * `links` - stackoverflow links.
///
/// # Returns
/// A list of links with splitter.  Which can directly output by the caller.
fn answers_links_only(links: &Vec<String>, restricted_length: usize) -> String {
    let mut results: Vec<String> = Vec::new();
    let length = links.len();
    let mut index: usize = 0;
    while index < length && index < restricted_length {
        let link = &links[index as usize];
        if !link.contains("question") {
            continue;
        }
        let url: Url = Url::parse(link)
            .expect("Parse url failed, if you receive this message, please fire an issue.");

        let answer: String = format!("Title - {}\n{}", extract_question(url.path()), *link,);
        results.push(answer);
        index += 1;
    }
    return results.join(SPLITTER);
}

/// Extract question content.
///
/// # Example
/// let question: &str = extract_question("questions/user_id/the-specific-question")
/// assert_eq!(question, String::from("the specific question"))
fn extract_question(path: &str) -> String {
    // The stack overflow question have the following format
    // https://stackoverflow.com/questions/user_id/the-specific-question
    // we want to extract the link out
    let splitted: Vec<&str> = path.split("/").collect();
    return splitted[splitted.len() - 1].replace("-", " ");
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
            "
        <html>
            <body>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <pre>
                            <code>println!(\"hello world\")</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        ",
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("println!(\"hello world\")"));
        }
    }

    #[test]
    fn test_parse_answer_when_pre_and_code_both_existed() {
        let page: String = String::from(
            "
        <html>
            <body>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <p>answer <code>goto</code> here </p>
                        <pre>
                            <code>println!(\"hello world\")</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        ",
        );
        let conf: Config = Config::new(OutputOption::OnlyCode, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("println!(\"hello world\")"));
        }
    }

    #[test]
    fn test_parse_answer_when_only_code_existed() {
        let page: String = String::from(
            "
        <html>
            <body>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <p>answer <code>goto</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        ",
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
            "
        <html>
            <body>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <p>answer <code>goto</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        ",
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let answer: Option<String> = parse_answer(page, &conf);

        assert_eq!(answer.is_some(), true);

        if let Some(code) = answer {
            assert_eq!(code.trim(), String::from("answer goto here"));
        }
    }

    #[test]
    fn test_parse_answer_when_two_answers_available() {
        let page: String = String::from(
            "
        <html>
            <body>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <p>answer <code>lower</code> here </p>
                    </div>
                </div>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">9000</div>
                    <div class=\"post-text\">
                        <p>answer <code>higher</code> here </p>
                    </div>
                </div>
            </body>
        </html>
        ",
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
            "
        <html>
            <body>
                <a class=\"post-tag\">python</a>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        ",
        );
        let conf: Config = Config::new(OutputOption::All, 1, false);
        let un_colorized_answer: String = parse_answer(page, &conf).unwrap();
        let conf: Config = Config::new(OutputOption::All, 1, true);
        let page: String = String::from(
            "
        <html>
            <body>
                <a class=\"post-tag\">python</a>
                <div class=\"answer\">
                    <div class=\"js-vote-count\">130</div>
                    <div class=\"post-text\">
                        <pre>
                            <code>print(1 + 2)</code>
                        </pre>
                    </div>
                </div>
            </body>
        </html>
        ",
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
}
