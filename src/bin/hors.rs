#[macro_use]
extern crate log;

use clap::{self, Parser};
use hors::{self, Config, Error, Output, OutputOption, PagingOption, Result, SearchEngine};

use reqwest::{Client, ClientBuilder};

use std::process;
use std::str::FromStr;

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
    /// just clear local hors cache.
    #[arg(long)]
    clear_cache: bool,
    /// display the full text of answer.
    #[arg(short, long)]
    all: bool,
    /// display only the answer link.
    #[arg(short, long)]
    link: bool,
    /// make raw output (not colorized)
    #[arg(short, long)]
    raw: bool,
    /// "specify how to page output, can be `auto`, `never`"
    #[arg(
        short,
        long,
        default_value = "auto",
    )]
    paging: String,
    /// number of answers to return.
    #[arg(
        short,
        long,
        default_value = "1",
    )]
    number_answers: u8,
    /// select middle search engine, currently support `bing`, `google`, `duckduckgo`, `stackoverflow`.
    #[arg(
        short,
        long,
        default_value = "duckduckgo",
        env = "HORS_ENGINE",
    )]
    engine: String,
    /// Disable system proxy.
    #[arg(short, long)]
    disable_proxy: bool,
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();
    let opts: Opts = Opts::parse();
    env_logger::init();
    if opts.clear_cache {
        if let Err(e) = hors::clear_local_cache() {
            eprintln!("clear local cache failed, reason: {:?}", e);
            process::exit(1);
        }
        process::exit(0);
    }

    let search_engine = SearchEngine::from_str(&opts.engine)?;
    debug!("Search under the {:?}", search_engine);

    // Initialize reqwest::Client instance.
    let mut client_builder: ClientBuilder = reqwest::ClientBuilder::new().cookie_store(true);
    if opts.disable_proxy {
        debug!("disable proxy");
        client_builder = client_builder.no_proxy();
    }
    let client: Client = client_builder.build().unwrap_or_else(|err| {
        eprintln!("Build client failed: {}", err);
        process::exit(1);
    });

    let target_links: Vec<String> =
        hors::search_links_with_client(&opts.query.join(" "), search_engine, &client)
            .await
            .unwrap_or_else(|err| {
                if let Error::Parse(_) = err {
                    eprintln!(
                        "Search stackoverflow link failed with '{:?}' search engine, \
                         you can try another engine through `-e` argument, or specify `$HORS_ENGINE` env variable to another value", search_engine
                    );
                } else {
                    eprintln!("Run query failed with '{:?}' search engine, error message: {}, \
                    you can try another engine through `-e` argument, or specify `$HORS_ENGINE` env variable to another value", search_engine, err);
                }
                process::exit(1);
            });

    let conf: Config = init_config(&opts);
    debug!("User config: {:?}", conf);
    let answers: String = hors::get_answers_with_client(&target_links, conf, client)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Hors is running to error: {}", err);
            process::exit(1);
        });
    let answers: String = format!("\n\n{}", answers);

    // create an output object and get an output handler, use the handler to handle our result.
    let paging_option = PagingOption::from_str(&opts.paging).unwrap_or(PagingOption::Auto);
    let mut output = Output::new(&paging_option);
    let handler = output.get_handler();
    handler.write_all(answers.as_bytes()).expect("success");
    Ok(())
}

/// initialize config from user input arguments.
fn init_config(opts: &Opts) -> Config {
    let output_option = if opts.link {
        OutputOption::Links
    } else if opts.all {
        OutputOption::All
    } else {
        OutputOption::OnlyCode
    };

    Config::new(output_option, opts.number_answers, !opts.raw)
}
