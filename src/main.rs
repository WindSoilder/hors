extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::error::Error;

mod engine;
mod error;
mod outputer;
mod utils;

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
                .help("number of answers to return"),
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

fn main() -> Result<(), Box<Error>> {
    let matches: ArgMatches = parser_matches();

    let target_links: Vec<String> =
        engine::bing::search(&String::from(matches.value_of("QUERY").unwrap()))?;
    if matches.is_present("link") {
        let parsed_links: String = outputer::get_results_with_links_only(&target_links);
        println!("{}", parsed_links);
    } else if matches.is_present("all") {

    }

    return Ok(());
}
