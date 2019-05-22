use crate::error::Result;
use bincode::{deserialize_from, serialize_into};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// The answer record relative information is integrated here.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AnswerRecord {
    /// the url contains useful links.
    link: String,
    /// tags for the link, it may contains useful topic about the question.
    tags: Vec<String>,
    /// actual answer for the question.
    answer: String,
    /// when it was created.
    created_time: u128,
}

impl AnswerRecord {
    pub fn new(link: String, tags: Vec<String>, answer: String) -> AnswerRecord {
        return AnswerRecord {
            link,
            tags,
            answer,
            created_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went beckwards")
                .as_millis(),
        };
    }

    pub fn link(&self) -> &String {
        return &self.link;
    }

    pub fn tags(&self) -> &Vec<String> {
        return &self.tags;
    }

    pub fn answer(&self) -> &String {
        return &self.answer;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AnswerRecordsCache(HashMap<String, AnswerRecord>);

impl AnswerRecordsCache {
    /// Load answers into cache.
    ///
    /// # Return value
    ///
    /// Return the instance of AnswerRecordsCache.
    pub fn load() -> AnswerRecordsCache {
        /*
        TODO:
        try to get answer files from $HOME/.hors/.answers
        if we can't find the file
            just return an empty AnswerRecords
        else
            deserialize the file and load it to answer records
        */
        return AnswerRecordsCache(HashMap::new());
    }

    /// Get answer from the given link.
    ///
    /// # Arguments
    ///
    /// * `link` - link contains stackoverflow question.
    ///
    /// # Return value
    /// Return cached answer record if we can find it, else returns None.
    pub fn get(&self, link: &String) -> Option<AnswerRecord> {
        /*
        TODO:
        try to get answer according to specific link
        if we can find relative record
            just return it
        Else
            we return nothing
        */
        return None;
    }

    /// Put answer to cache.
    ///
    /// # Arguments
    ///
    /// * `answer` - answer information.
    pub fn put(&self, answer: AnswerRecord) {
        /*
        TODO:
        extract out link from answer
        then save answer to inner hashmap
        */
    }

    /// Save the data into local file.
    ///
    /// # Return value
    ///
    /// Returns Ok if save success, else return an error.
    pub fn save(&self) -> Result<()> {
        /*
        TODO:
        if the inner size of answer records is too large
            truncate it to have size 100
        dump answer to spefic file $HOME/.hors/.answers
        if we can dump successfully
            return ok with nothing
        Else
            propogate error message out.
        */
        return Ok(());
    }
}
