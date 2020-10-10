//! Crawler for fetching so page.

use super::records::AnswerRecordsCache;
use crate::config::Config;
use crate::utils::random_agent;
use crate::Result;
use reqwest::{Client, Response};
use std::process;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub struct PageCrawler {
    links: Vec<String>,
    conf: Config,
    records_cache: AnswerRecordsCache,
    client: Client,
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

            for t in tasks {
                // fetch result from sub tasks and save them to cache.
                if let Ok(crawled_result) = t.await {
                    if let Ok(crawled_data) = crawled_result {
                        let (link, page) = crawled_data.into_cache_item();
                        self.records_cache.put(link, page);
                    }
                }
            }

            // when hors gets what we wanted answer, save it for next time using.
            if let Err(err) = self.records_cache.save() {
                warn!(
                    "Can't save cache into local directory, error msg: {:?}",
                    err
                );
            }

            if self.msg_sender.send(CrawlerMsg::Done).await.is_err() {
                error!(
                    "Receiver is dropped un-expectly, if you see this message, please fire an issue"
                );
                process::exit(1);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub enum CrawlerMsg {
    Data(CrawledData),
    Done,
}

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
