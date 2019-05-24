use crate::error::Result;
use bincode::{deserialize_from, serialize_into};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// The answer record relative information is integrated here.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct AnswerRecord {
    /// the url contains useful links.
    link: String,
    /// actual page for the link.
    page: String,
    /// when it was created.
    created_time: u64,
}

impl AnswerRecord {
    pub fn new(link: String, page: String) -> AnswerRecord {
        return AnswerRecord {
            link,
            page,
            created_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went beckwards")
                .as_secs(),
        };
    }

    pub fn link(&self) -> &String {
        return &self.link;
    }

    pub fn page(&self) -> &String {
        return &self.page;
    }

    pub fn is_too_old(&self) -> bool {
        const HALF_MONTH_IN_SECONDS: u64 = 15 * 24 * 3600;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went beckwards")
            .as_secs();
        return (current_time - self.created_time) > HALF_MONTH_IN_SECONDS;
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
    /// Return cached page if we can find it, else returns None.
    pub fn get(&self, link: &String) -> Option<&String> {
        let possible_page: Option<&AnswerRecord> = self.0.get(link);
        match possible_page {
            // if we can find relative record
            Some(record) => {
                // check if the record is too old
                if record.is_too_old() {
                    return None;
                }
                return Some(&record.page);
            }
            None => None,
        }
    }

    /// Put answer to cache.
    ///
    /// If the link is already in cache, the content page is replaced.
    ///
    /// # Arguments
    ///
    /// * `link` - link to page.
    /// * `page` - the content of page.
    pub fn put(&mut self, link: String, page: String) {
        let record: AnswerRecord = AnswerRecord::new(link.clone(), page);
        self.0.insert(link, record);
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
