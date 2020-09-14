#[macro_use]
extern crate log;

use clap::{self, Clap};
use hors::{self, Config, Error, Output, OutputOption, PagingOption, Result, SearchEngine};

use reqwest::{Client, ClientBuilder};

use std::process;
use std::str::FromStr;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
    #[clap(short, long, about("display the full text of answer."))]
    all: bool,
    #[clap(short, long, about("display only the answer link."))]
    link: bool,
    #[clap(short, long, about("make raw output (not colorized)."))]
    raw: bool,
    #[clap(
        short,
        long,
        default_value = "auto",
        about("specify how to page output, can be `auto`, `never`")
    )]
    paging: String,
    #[clap(
        short,
        long,
        default_value = "1",
        about("number of answers to return.")
    )]
    number_answers: u8,
    #[clap(
        short,
        long,
        default_value = "duckduckgo",
        env = "HORS_ENGINE",
        about("select middle search engine, currently support `bing`, `google`, `duckduckgo`, `stackoverflow`.")
    )]
    engine: String,
    #[clap(short, long, about("Disable system proxy."))]
    disable_proxy: bool,
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();
    let opts: Opts = Opts::parse();

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
    let answers: String = hors::get_answers_with_client(&target_links, conf, &client)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Hors is running to error: {}", err);
            process::exit(1);
        });

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
