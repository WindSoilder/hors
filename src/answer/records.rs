use crate::error::Result;
use bincode::{deserialize_from, serialize_into};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;
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
        AnswerRecord {
            link,
            page,
            created_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went beckwards")
                .as_secs(),
        }
    }

    /// Make desicion that if this AnswerRecord is too old.
    ///
    /// If the self object lives longer than(not longer than equal) half
    /// month, it's too old.
    ///
    /// # Arguments
    ///
    /// * `time` - given timestamp of time as seconds.
    ///
    /// # Returns
    ///
    /// Return true if the object is too old.
    pub fn is_too_old(&self, time: u64) -> bool {
        const HALF_MONTH_IN_SECONDS: u64 = 15 * 24 * 3600;
        (time - self.created_time) > HALF_MONTH_IN_SECONDS
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AnswerRecordsCache(HashMap<String, AnswerRecord>);

impl AnswerRecordsCache {
    /// Load answers into cache.
    ///
    /// # Returns
    ///
    /// Return the instance of AnswerRecordsCache.  Error will be returned if
    /// loading local cache file failed.
    pub fn load() -> Result<AnswerRecordsCache> {
        if let Some(base_dirs) = BaseDirs::new() {
            // just create cache file if not existed, and deserialize it.
            let dir = base_dirs.cache_dir().to_path_buf();
            let cache_file: PathBuf = AnswerRecordsCache::create_file_if_not_existed(&dir)?;
            let f = File::open(cache_file)?;
            let answer_records: AnswerRecordsCache = deserialize_from(f)?;
            return Ok(answer_records);
        }
        Ok(AnswerRecordsCache(HashMap::new()))
    }

    /// Create cache with no records.
    ///
    /// # Returns
    ///
    /// An empty cache.
    pub fn load_empty() -> AnswerRecordsCache {
        AnswerRecordsCache(HashMap::new())
    }

    /// Get answer from the given link.
    ///
    /// # Arguments
    ///
    /// * `link` - link contains stackoverflow question.
    ///
    /// # Returns
    /// Return cached page if we can find it and it's not too old, else returns None.
    pub fn get(&self, link: &str) -> Option<&String> {
        let possible_page: Option<&AnswerRecord> = self.0.get(link);
        match possible_page {
            // if we can find relative record
            Some(record) => {
                // check if the record is too old
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went beckwards")
                    .as_secs();
                if record.is_too_old(current_time) {
                    return None;
                }
                Some(&record.page)
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
    /// # Returns
    ///
    /// Returns Ok if save success, else return an error.
    pub fn save(&self) -> Result<()> {
        const MAX_SIZE: usize = 300;
        // if the inner size of answer records is too large
        if MAX_SIZE < self.0.len() {
            // TODO: truncate it to have size MAX_SIZE
        }
        if let Some(base_dirs) = BaseDirs::new() {
            let dir = base_dirs.cache_dir();
            let cache_path: PathBuf = dir.join("hors").join("answers");
            let f = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(cache_path)?;
            // dump answer to spefic file $CACHE/hors/answers
            serialize_into(f, self).unwrap();
        }
        Ok(())
    }

    fn create_file_if_not_existed(dir: &PathBuf) -> Result<PathBuf> {
        let cache_directory: PathBuf = dir.join("hors");
        if !cache_directory.exists() {
            create_dir_all(&cache_directory).unwrap();
        }

        let answers = cache_directory.join("answers");
        if !answers.exists() {
            File::create(&answers)?;
        }
        Ok(answers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_record_initialize() {
        let link: String = "http://test_link".to_string();
        let page: String = "<html></html>".to_string();
        let current_time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went beckwards")
            .as_secs();
        let test_record = AnswerRecord::new(link, page);
        // we should expect the test_record's create_time - current_time
        // is less than 1.
        assert!(test_record.created_time - current_time < 1)
    }

    #[test]
    fn test_answer_record_too_old() {
        let link: String = "http://test_link".to_string();
        let page: String = "<html></html>".to_string();
        let current_time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went beckwards")
            .as_secs();
        let test_record = AnswerRecord::new(link, page);
        assert_eq!(test_record.is_too_old(current_time), false);

        let half_month_and_one_second: u64 = 3600 * 24 * 15 + 1;
        let time_after_half_month: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went beckwards")
            .as_secs()
            + half_month_and_one_second;
        assert_eq!(test_record.is_too_old(time_after_half_month), true);
    }

    #[test]
    fn test_answer_record_cache_empty() {
        let record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        assert_eq!(record_cache.0.is_empty(), true);
    }

    #[test]
    fn test_answer_record_put() {
        let mut record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        record_cache.put("http://test_link".to_string(), "<html></html>".to_string());
        assert_eq!(
            record_cache.get(&String::from("http://test_link")),
            Some(&String::from("<html></html>"))
        );
    }

    #[test]
    fn test_answer_record_put_if_key_is_already_existed() {
        let mut record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        record_cache.put("http://test_link".to_string(), "<html></html>".to_string());
        record_cache.put("http://test_link".to_string(), "<html>2</html>".to_string());
        assert_eq!(
            record_cache.get(&String::from("http://test_link")),
            Some(&String::from("<html>2</html>"))
        );
    }

    #[test]
    fn test_answer_record_get() {
        let mut record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        record_cache.put("http://test_link".to_string(), "<html></html>".to_string());
        assert_eq!(
            record_cache.get(&String::from("http://test_link")),
            Some(&String::from("<html></html>"))
        );
    }

    #[test]
    fn test_answer_record_get_when_key_is_not_existed() {
        let record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        assert_eq!(
            record_cache
                .get(&String::from("http://test_link"))
                .is_none(),
            true
        );
    }
}
