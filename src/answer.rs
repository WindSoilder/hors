//! This module contains api to get results from stack overflow page
use crate::config::{Config, OutputOption};
use crate::error::Result;
use crate::utils::random_agent;
use reqwest::Url;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

// TODO: Add docstring
pub fn get_answers(links: &Vec<String>, conf: Config) -> Result<String> {
    match conf.option() {
        OutputOption::All => return get_detailed_answer(links, conf),
        _ => return Ok(get_results_with_links_only(links, conf)),
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
                    None => results.push(format!("Can't get answer from {}", link))
                }
            },
            None => break,
        }
    }
    return Ok(results.join("\n==========\n"));
}

fn parse_answer(page: String, config: &Config) -> Option<String> {
    let doc: Document = Document::from(page.as_str());
    let mut first_answer = doc.find(Class("answer"));

    if let Some(answer) = first_answer.next() {
        // TODO: Add links to the answer.  And format the code.
        if let Some(instruction) = answer.find(Class("post-text")).next() {
            return Some(instruction.text());
        }
    }
    return None;
}

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
fn get_results_with_links_only(links: &Vec<String>, conf: Config) -> String {
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
