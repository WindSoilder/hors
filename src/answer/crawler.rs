//! Crawler to fetch stackoverflow page.
//!
//! Simple usage example:
//!
//! ```ignore
//! use tokio::sync::mpsc;
//! use crate::config::{Config, OutputOption};
//! use reqwest::ClientBuilder;
//!
//! let conf = Config::new(OutputOption::All, 1, false);
//! let client = ClientBuilder::new().cookie_store(true).build().unwrap();
//!
//! let (tx, mut rx): (Sender<CrawlerMsg>, Receiver<CrawlerMsg>) = mpsc::channel(10);
//! let page_crawler = PageCrawler::new(links, conf, client, tx);
//! page_crawler.fetch();
//!
//! while let Some(page) = rx.recv().await {
//!     match page {
//!         CrawlerMsg::Done => break,
//!         CrawlerMsg::Data(m) => {
//!             // handle for crawled data.
//!         }
//!     }
//! };
//!

use super::records::AnswerRecordsCache;
use crate::config::Config;
use crate::utils::random_agent;
use crate::Result;
use reqwest::{Client, Response};
use std::process;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

/// Crawler to fetch stackoverflow page
pub struct PageCrawler {
    /// so links to fetch relative page.
    links: Vec<String>,
    /// current configuration.
    conf: Config,
    /// relative page cache, we can use it to avoid too much network traffic.
    records_cache: AnswerRecordsCache,
    /// reqwest http client.
    client: Client,
    /// message sender, which is used to communicate with crawler user.
    msg_sender: Sender<CrawlerMsg>,
}

impl PageCrawler {
    pub fn new(
        links: Vec<String>,
        conf: Config,
        client: Client,
        msg_sender: Sender<CrawlerMsg>,
    ) -> PageCrawler {
        debug!("Try to load cache from local cache file.");
        let load_result: Result<AnswerRecordsCache> = AnswerRecordsCache::load();
        let records_cache: AnswerRecordsCache = match load_result {
            Ok(cache) => cache,
            Err(err) => {
                warn!("Can't load cache from local cache file, errmsg {:?}", err);
                AnswerRecordsCache::load_empty()
            }
        };
        debug!("Load cache complete.");

        PageCrawler {
            links,
            conf,
            records_cache,
            client,
            msg_sender,
        }
    }

    /// Consume self to make a concurrent fetch.
    ///
    /// All the fetched pages will be send through `self.msg_sender`.
    pub fn fetch(mut self) {
        tokio::spawn(async move {
            let mut links_iter = self.links.into_iter();
            let mut tasks = vec![];
            for _ in 0..self.conf.numbers() {
                let mut sender = self.msg_sender.clone();
                let next_link = links_iter.next();
                match next_link {
                    Some(link) => {
                        // the given links may contains the url doesn't contains `question`
                        // tag, so it's not a question, just deal with nothing to it.
                        if !link.contains("question") {
                            continue;
                        }

                        // try to fetch page from cache first.
                        let page_from_cache: Option<&String> = self.records_cache.get(&link);
                        match page_from_cache {
                            // When we can get answer from cache, just send the result back to upload task.
                            Some(page) => {
                                // send crawled data to other side.
                                if sender
                                    .send(CrawlerMsg::Data(CrawledData::new(
                                        format!("- Answer from {}", &link),
                                        link,
                                        page.into(),
                                    )))
                                    .await
                                    .is_err()
                                {
                                    error!(
                                            "Receiver is dropped un-expectly, if you see this message, please fire an issue"
                                        );
                                    process::exit(1);
                                }
                                continue;
                            }
                            None => {
                                let work_client: Client = self.client.clone();
                                // spawn task only we can't find page from cache.
                                let single_fetcher: JoinHandle<Result<CrawledData>> = tokio::spawn(
                                    async move {
                                        let page: String = get_page(&link, &work_client).await?;
                                        let crawled_data = CrawledData::new(
                                            format!("- Answer from {}", &link),
                                            link.clone(),
                                            page.clone(),
                                        );

                                        // send crawled data to other side.
                                        if sender
                                            .send(CrawlerMsg::Data(crawled_data.clone()))
                                            .await
                                            .is_err()
                                        {
                                            error!(
                                                "Receiver is dropped un-expectly, if you see this message, please fire an issue"
                                            );
                                            process::exit(1);
                                        }
                                        return Ok(crawled_data);
                                    },
                                );
                                tasks.push(single_fetcher);
                            }
                        }
                    }
                    None => break,
                }
            }

            // Wait for all sub-tasks complete, push result to our cache.
            for t in tasks {
                // Ignore any sub tasks error.
                if let Ok(crawled_result) = t.await {
                    if let Ok(crawled_data) = crawled_result {
                        let (link, page) = crawled_data.into_cache_item();
                        self.records_cache.put(link, page);
                    }
                }
            }

            // when hors gets what we wanted answer, save cache for next time using.
            if let Err(err) = self.records_cache.save() {
                warn!(
                    "Can't save cache into local directory, error msg: {:?}",
                    err
                );
            }

            // Notification done message.
            if self.msg_sender.send(CrawlerMsg::Done).await.is_err() {
                error!(
                    "Receiver is dropped un-expectly, if you see this message, please fire an issue"
                );
                process::exit(1);
            }
        });
    }
}

/// The message which is used to communicate with other task.
#[derive(Debug, Clone)]
pub enum CrawlerMsg {
    Data(CrawledData),
    Done,
}

/// Data crawled by our crawler
#[derive(Debug, Clone)]
pub struct CrawledData {
    link: String,
    title: String,
    page: String,
}

impl CrawledData {
    pub fn new(title: String, link: String, page: String) -> CrawledData {
        CrawledData { link, title, page }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_link(&self) -> &str {
        &self.title
    }

    pub fn get_page(&self) -> &str {
        &self.page
    }

    pub fn into_cache_item(self) -> (String, String) {
        (self.link, self.page)
    }
}

async fn get_page(link: &str, client: &Client) -> Result<String> {
    let resp: Response = client
        .get(link)
        .header(reqwest::header::USER_AGENT, random_agent())
        .send()
        .await?;
    debug!("Response status from stackoverflow: {:?}", resp);
    let page: String = resp.text().await?;
    Ok(page)
}
