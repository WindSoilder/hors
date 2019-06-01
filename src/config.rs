use crate::error::{HorsError, Result};
use std::str::FromStr;

#[derive(Debug)]
/// The results output options is defined here.
pub enum OutputOption {
    /// Only output links.
    Links,
    /// Output answer details, which contains code and plain text.
    All,
    /// Only output code in answer.
    OnlyCode,
}

#[derive(Debug)]
/// supported search engine definition
pub enum SearchEngine {
    /// Microsoft bing search engine.
    Bing,
    /// Google search engine.
    Google,
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
    pub fn new(output_option: OutputOption, numbers: u8, colorize: bool) -> Self {
        return Config {
            option: output_option,
            numbers,
            colorize,
        };
    }

    pub fn option(&self) -> &OutputOption {
        return &self.option;
    }

    pub fn numbers(&self) -> u8 {
        return self.numbers;
    }

    pub fn colorize(&self) -> bool {
        return self.colorize;
    }
}

impl FromStr for SearchEngine {
    type Err = HorsError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "bing" => return Ok(SearchEngine::Bing),
            // FIXME: There are issues with google search engine, should fixed in the future.
            // "google" => return Ok(SearchEngine::Google),
            _ => return Err(HorsError::from_parse("Not supported search engine")),
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
        // FIXME: There are issues with google search engine, should fixed in the future.
        // let search_engine = SearchEngine::from_str("google");
        // assert_eq!(search_engine.is_ok(), true);
    }

    #[test]
    fn test_search_engine_from_invalid_str() {
        let search_engine = SearchEngine::from_str("what's this?");
        assert_eq!(search_engine.is_err(), true);
    }
}
