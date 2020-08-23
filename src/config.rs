use crate::error::{Error, Result};
use std::str::FromStr;

#[derive(Debug)]
/// The results output options.
pub enum OutputOption {
    /// Only output links.
    Links,
    /// Output answer details, which contains code and plain text.
    All,
    /// Only output code in answer.
    OnlyCode,
}

#[derive(Debug, Clone, Copy)]
/// supported search engine definition.
pub enum SearchEngine {
    /// Microsoft bing search engine.
    Bing,
    /// Google search engine.
    Google,
    /// DuckDuckGo search engine.
    DuckDuckGo,
    /// Stackoverflow internal search.
    StackOverflow,
}

#[derive(Debug)]
/// The user config information is integrated here.
pub struct Config {
    /// Terminal output options.
    option: OutputOption,
    /// The number of answers to be output.
    numbers: u8,
    /// Indicate that the output code shoule be colorized or not.
    colorize: bool,
}

impl Config {
    pub fn new(output_option: OutputOption, numbers: u8, colorize: bool) -> Config {
        Config {
            option: output_option,
            numbers,
            colorize,
        }
    }

    pub fn option(&self) -> &OutputOption {
        &self.option
    }

    pub fn numbers(&self) -> u8 {
        self.numbers
    }

    pub fn colorize(&self) -> bool {
        self.colorize
    }
}

impl FromStr for SearchEngine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bing" => Ok(SearchEngine::Bing),
            "google" => Ok(SearchEngine::Google),
            "duckduckgo" => Ok(SearchEngine::DuckDuckGo),
            "stackoverflow" => Ok(SearchEngine::StackOverflow),
            _ => Err(Error::from_parse("Not supported search engine")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_engine_from_str() {
        let search_engine = SearchEngine::from_str("bing");
        assert_eq!(search_engine.is_ok(), true);
        let search_engine = SearchEngine::from_str("google");
        assert_eq!(search_engine.is_ok(), true);
    }

    #[test]
    fn test_search_engine_from_invalid_str() {
        let search_engine = SearchEngine::from_str("what's this?");
        assert_eq!(search_engine.is_err(), true);
    }
}
