//! This module contains api to get results from stack overflow page
use crate::config::{Config, OutputOption};
use crate::error::Result;
use crate::utils::random_agent;
use reqwest::Url;
use select::document::Document;
use select::predicate::{Class, Name};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// TODO: Add docstring
pub fn get_answers(links: &Vec<String>, conf: Config) -> Result<String> {
    match conf.option() {
        OutputOption::Links => return Ok(get_results_with_links_only(links)),
        _ => return get_detailed_answer(links, conf),
    }
}

// TODO: Add docstring
pub fn get_detailed_answer(links: &Vec<String>, conf: Config) -> Result<String> {
    let mut results: Vec<String> = Vec::new();
    let user_agent: &str = random_agent();
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    let mut links_iter = links.iter();

    for _ in 0..conf.numbers() {
        let next_link = links_iter.next();
        match next_link {
            Some(link) => {
                if !link.contains("question") {
                    continue;
                }
                let page: String = client
                    .get(link)
                    .header(reqwest::header::USER_AGENT, user_agent)
                    .send()?
                    .text()?;

                let answer = parse_answer(page, &conf);
                match answer {
                    Some(content) => results.push(content),
                    None => results.push(format!("Can't get answer from {}", link)),
                }
            }
            None => break,
        }
    }
    return Ok(results.join("\n==========\n"));
}

fn parse_answer(page: String, config: &Config) -> Option<String> {
    let doc: Document = Document::from(page.as_str());
    // The question tags may contains useful information about the language topic.
    let mut question_tags: Vec<String> = vec![];
    let tags = doc.find(Class("post-tag"));
    for tag in tags {
        question_tags.push(tag.text());
    }

    let mut first_answer = doc.find(Class("answer"));

    if let Some(answer) = first_answer.next() {
        // TODO: Add links to the answer.  And format the code.
        match *config.option() {
            OutputOption::OnlyCode => {
                return parse_answer_instruction(answer, question_tags, config.colorize());
            }
            OutputOption::All => {
                return parse_answer_detailed(answer, question_tags, config.colorize());
            }
            _ => panic!("parse_answer shoudn't get config with OutputOption::Link"),
        }
    }
    return None;
}

fn parse_answer_instruction(
    answer_node: select::node::Node,
    question_tags: Vec<String>,
    should_colorize: bool,
) -> Option<String> {
    if let Some(code_instruction) = answer_node.find(Name("code")).next() {
        if should_colorize {
            return Some(colorized_code(code_instruction.text(), question_tags));
        } else {
            return Some(code_instruction.text());
        }
    }
    if let Some(title) = answer_node.find(Name("pre")).next() {
        if should_colorize {
            return Some(colorized_code(title.text(), question_tags));
        } else {
            return Some(title.text());
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
        return Some(instruction.text());
    }
    return None;
}

/// make code block colorized.
///
/// Note that this function should only accept code block.
fn colorized_code(code: String, possible_tags: Vec<String>) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts: ThemeSet = ThemeSet::load_defaults();
    let syntax: &SyntaxReference = guess_syntax(possible_tags, &ss);
    let mut h = HighlightLines::new(&syntax, &ts.themes["base16-ocean.dark"]);
    let mut colorized: String = String::new();

    for line in LinesWithEndings::from(code.as_str()) {
        let escaped = as_24_bit_terminal_escaped(&h.highlight(line, &ss), true);
        colorized = colorized + escaped.as_str();
    }
    return colorized;
}

/// &SyntaxReference
fn guess_syntax(possible_tags: Vec<String>, ss: &SyntaxSet) -> &SyntaxReference {
    for tag in possible_tags {
        let syntax = ss.find_syntax_by_token(tag.as_str());
        if let Some(result) = syntax {
            return result;
        }
    }
    return ss.find_syntax_plain_text();
}

//??? Why the following code doesn't work
// fn guess_syntax2(possible_tags: Vec<String>) -> &SyntaxReference {
//     let ss = SyntaxSet::load_defaults_newlines();
//     for tag in possible_tags {
//         let syntax = ss.find_syntax_by_token(tag.as_str());
//         if let Some(result) = syntax {
//             // ??? Why I can't return a SyntaxReference
//             return result;
//         }
//     }
//     return ss.find_syntax_plain_text();
// }

//??? Why the following code doesn't work either
// fn guess_syntax3(possible_tags: Vec<String>) -> SyntaxReference {
//     let ss = SyntaxSet::load_defaults_newlines();
//     for tag in possible_tags {
//         let syntax = ss.find_syntax_by_token(tag.as_str());
//         if let Some(result) = syntax {
//             // ??? Why I can't return a SyntaxReference
//             return *result;
//         }
//     }
//     return *ss.find_syntax_plain_text();
// }

// TODO: Give it more reasonable name.
/// output links from the given stackoverflow links.
///
///
/// # Arguments
///
/// * `links` - stackoverflow links.
///
/// # Returns
/// A list of links with splitter.  Which can directly output by the caller.
fn get_results_with_links_only(links: &Vec<String>) -> String {
    let mut results: Vec<String> = Vec::new();
    for link in links.iter() {
        if !link.contains("question") {
            continue;
        }
        let url: Url = Url::parse(link)
            .expect("Parse url failed, if you receive this message, please fire an issue.");

        let answer: String = format!(
            "Title - {}\n{}\n\n{}\n",
            extract_question(url.path()),
            *link,
            "============="
        );
        results.push(answer);
    }
    return results.join("\n");
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
