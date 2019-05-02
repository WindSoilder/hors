use reqwest::Url;

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
pub fn get_results_with_links_only(links: &Vec<String>) -> String {
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
