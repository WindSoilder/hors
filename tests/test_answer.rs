use reqwest::Client;

use hors::answer::{get_answers_with_client, SPLITTER};
use hors::config::{Config, OutputOption};

#[tokio::test]
async fn test_get_answers_with_links_only() {
    let links: Vec<String> = vec![String::from(
        "https://stackoverflow.com/questions/7771011/parse-json-in-python",
    )];
    let conf: Config = Config::new(OutputOption::Links, 10, false);
    let client: Client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();
    let answers: String = get_answers_with_client(&links, conf, client)
        .await
        .expect("Get answer through stackoverflow should success")
        .split(SPLITTER)
        .collect();
    // each answer itself only contains Title: xxx\nhttps://stackoverflow.com/xx
    assert_eq!(
        answers,
        "Title - parse json in python
https://stackoverflow.com/questions/7771011/parse-json-in-python"
    );
}

#[tokio::test]
async fn test_get_answers_with_detailed_option() {
    let links: Vec<String> = vec![String::from(
        "https://stackoverflow.com/questions/7771011/parse-json-in-python",
    )];
    let conf: Config = Config::new(OutputOption::All, 10, false);
    let client: Client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();
    let answers: String = get_answers_with_client(&links, conf, client)
        .await
        .expect("Get answer through stackoverflow should success")
        .split(SPLITTER)
        .collect();

    assert_eq!(
        answers.trim(),
        r#"- Answer from https://stackoverflow.com/questions/7771011/parse-json-in-python

Very simple:
import json
data = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')
print(data['two'])  # or `print data['two']` in Python 2"#
    )
}

#[tokio::test]
async fn test_get_answers_with_instruction() {
    let links: Vec<String> = vec![String::from(
        "https://stackoverflow.com/questions/7771011/parse-json-in-python",
    )];
    let conf: Config = Config::new(OutputOption::OnlyCode, 10, false);
    let client: Client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap();
    let answers: String = get_answers_with_client(&links, conf, client)
        .await
        .expect("Get answer through stackoverflow should success")
        .split(SPLITTER)
        .collect();
    assert_eq!(
        answers,
        r#"- Answer from https://stackoverflow.com/questions/7771011/parse-json-in-python
import json
data = json.loads('{"one" : "1", "two" : "2", "three" : "3"}')
print(data['two'])  # or `print data['two']` in Python 2
"#
    )
}
