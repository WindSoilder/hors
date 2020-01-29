#[macro_use]
extern crate log;

use clap::{App, Arg, ArgMatches};
use hors::{self, Config, OutputOption, Result, SearchEngine};
use reqwest::{Client, ClientBuilder};

use std::process;
use std::str::FromStr;

fn parser_matches<'a>() -> ArgMatches<'a> {
    let parser = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("all")
                .long("all")
                .short("a")
                .help("display the full text of the answer."),
        )
        .arg(
            Arg::with_name("link")
                .long("link")
                .short("l")
                .help("display only the answer link."),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .short("c")
                .help("enable colorized output."),
        )
        .arg(
            Arg::with_name("number_answers")
                .long("number_answers")
                .short("n")
                .takes_value(true)
                .default_value("1")
                .help("number of answers to return."),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .short("e")
                .takes_value(true)
                .default_value("duckduckgo")
                .help("select middle search engine, currently support `bing`, `google` and `duckduckgo`."),
        )
        .arg(
            Arg::with_name("disable_proxy")
                .long("disable_proxy")
                .help("Disable system proxy."),
        )
        .arg(Arg::with_name("QUERY").required(true));
    parser.get_matches()
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches: ArgMatches = parser_matches();

    let search_engine =
        SearchEngine::from_str(matches.value_of("engine").unwrap_or_default()).unwrap();
    debug!("Search under the {:?}", search_engine);

    // Initialize reqwest::Client instance.
    let mut client_builder: ClientBuilder = reqwest::ClientBuilder::new().cookie_store(true);
    if matches.is_present("disable_proxy") {
        client_builder = client_builder.no_proxy();
    }
    let client: Client = client_builder.build().unwrap_or_else(|err| {
        eprintln!("Build client failed: {}", err);
        process::exit(1);
    });

    let target_links: Vec<String> =
        hors::search_links_with_client(matches.value_of("QUERY").unwrap(), search_engine, &client)
            .await
            .unwrap_or_else(|err| {
                eprintln!("Search for target link failed: {}", err);
                process::exit(1);
            });

    let conf: Config = init_config(&matches);
    debug!("User config: {:?}", conf);
    let answers: String = hors::get_answers_with_client(&target_links, conf, &client)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Hors is running to error: {}", err);
            process::exit(1);
        });
    println!("{}", answers);

    Ok(())
}

/// initialize config from user input arguments.
fn init_config(matches: &ArgMatches) -> Config {
    let output_option = if matches.is_present("link") {
        OutputOption::Links
    } else if matches.is_present("all") {
        OutputOption::All
    } else {
        OutputOption::OnlyCode
    };

    Config::new(
        output_option,
        matches
            .value_of("number_answers")
            .unwrap_or_default()
            .parse::<u8>()
            .unwrap(),
        matches.is_present("color"),
    )
}
