#[macro_use]
extern crate log;
mod answer;
mod config;
mod engine;
mod error;
mod utils;

use clap::{App, Arg, ArgMatches};
use config::{Config, OutputOption, SearchEngine};
use std::error::Error;
// ??? why use std::str::FromStr is required.
use std::str::FromStr;

fn parser_matches<'a>() -> ArgMatches<'a> {
    let parser = App::new("hors")
        .author("WindSoilder, WindSoilder@outlook.com")
        .version("0.1.0")
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
                .help("enable colorized output"),
        )
        .arg(
            Arg::with_name("number_answers")
                .long("number_answers")
                .short("n")
                .takes_value(true)
                .default_value("1")
                .help("number of answers to return"),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .short("e")
                .takes_value(true)
                .default_value("bing")
                .help("select middle search engine, currently support `bing` and `google`."),
        )
        .arg(
            Arg::with_name("version")
                .long("version")
                .short("v")
                .help("displays the current version of howdoi"),
        )
        .arg(Arg::with_name("QUERY").required(true));
    return parser.get_matches();
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = parser_matches();

    let search_engine =
        SearchEngine::from_str(matches.value_of("engine").unwrap_or_default()).unwrap();
    debug!("Search under the {:?}", search_engine);

    let target_links: Vec<String> = engine::search_links(
        &String::from(matches.value_of("QUERY").unwrap()),
        search_engine,
    )?;

    let conf: Config = init_config(&matches);
    debug!("User config: {:?}", conf);
    let answers: String = answer::get_answers(&target_links, conf)?;
    println!("{}", answers);

    return Ok(());
}

/// initialize config from user input arguments.
fn init_config(matches: &ArgMatches) -> Config {
    let output_option: OutputOption;
    if matches.is_present("link") {
        output_option = OutputOption::Links;
    } else if matches.is_present("all") {
        output_option = OutputOption::All;
    } else {
        output_option = OutputOption::OnlyCode;
    }
    let conf: Config = Config::new(
        output_option,
        matches
            .value_of("number_answers")
            .unwrap_or_default()
            .parse::<u8>()
            .unwrap(),
        matches.is_present("color"),
    );
    return conf;
}
