use crate::error::{Error, Result};
use bincode::{deserialize_from, serialize_into};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
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
    /// the cache hit counter.
    hit_count: u64,
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
            hit_count: 0,
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
        if let Ok(cache_dir) = Self::get_cache_dir() {
            let cache_file = AnswerRecordsCache::create_file_if_not_existed(&cache_dir)?;
            let f = File::open(cache_file)?;
            let answer_records: AnswerRecordsCache = deserialize_from(f)?;
            return Ok(answer_records);
        }
        Ok(AnswerRecordsCache(HashMap::new()))
    }

    fn get_cache_path() -> Result<PathBuf> {
        let mut cache_dir = Self::get_cache_dir()?;
        cache_dir.push("answers_v2");
        Ok(cache_dir)
    }

    /// Remove local cache if it's existed.
    pub fn clear() -> Result<()> {
        let cache_path = Self::get_cache_path()?;
        fs::remove_file(cache_path)?;
        Ok(())
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
    pub fn get(&mut self, link: &str) -> Option<&String> {
        let possible_page: Option<&mut AnswerRecord> = self.0.get_mut(link);
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
                // update hit count.
                record.hit_count += 1;
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
    pub fn save(&mut self) -> Result<()> {
        const MAX_SIZE: usize = 100;
        // if the inner size of answer records is too large
        let length = self.0.len();
        if MAX_SIZE < length {
            // extract out records link and hit_count, make them order_by hit_count.
            let mut link_hit_counter: Vec<(u64, String)> = self
                .0
                .iter()
                .map(|(k, v)| (v.hit_count, k.clone()))
                .collect();
            link_hit_counter.sort();
            // Just truncate data to MAX_SIZE / 2.
            for i in 0..MAX_SIZE / 2 {
                let (_, link) = &link_hit_counter[i];
                self.0.remove(link);
            }
        }
        if let Ok(cache_path) = Self::get_cache_path() {
            let f = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(cache_path)?;
            // dump answer to spefic file $CACHE/hors/answers
            serialize_into(f, self).unwrap();
        }
        Ok(())
    }

    fn get_cache_dir() -> Result<PathBuf> {
        match BaseDirs::new() {
            Some(base_dirs) => {
                let mut dir = base_dirs.cache_dir().to_path_buf();
                dir.push("hors");
                Ok(dir)
            }
            None => Err(Error::from_parse("get cache dir failed.")),
        }
    }

    fn create_file_if_not_existed(cache_directory: &PathBuf) -> Result<PathBuf> {
        if !cache_directory.exists() {
            fs::create_dir_all(&cache_directory).unwrap();
        }

        let answers = cache_directory.join("answers_v2");
        if !answers.exists() {
            File::create(&answers)?;
        }
        Ok(answers)
    }
}

/// Remove local cache file if it's existed.
pub fn clear_local_cache() -> Result<()> {
    AnswerRecordsCache::clear()
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
        let mut record_cache: AnswerRecordsCache = AnswerRecordsCache::load_empty();
        assert_eq!(
            record_cache
                .get(&String::from("http://test_link"))
                .is_none(),
            true
        );
    }
}
