use std::fs::read_to_string;
use std::process::ExitCode;

use ansi_term::Color::{Blue, Yellow};
use globset::{GlobBuilder, GlobSetBuilder};
use human_panic::setup_panic;
use ignore::WalkBuilder;
use regex::Regex;
use reqwest::blocking::Client;

use cli::Cli;

mod cli;

fn main() -> ExitCode {
    setup_panic!();

    let cli = Cli::new();
    let mut result = ExitCode::SUCCESS;

    let mut builder = GlobSetBuilder::new();
    let mut exclude_builder = GlobSetBuilder::new();

    for glob in cli.glob {
        builder.add(
            GlobBuilder::new(glob.as_str())
                .literal_separator(true)
                .build()
                .expect("Glob pattern should be correct"),
        );
    }

    for glob in cli.exclude {
        exclude_builder.add(
            GlobBuilder::new(glob.as_str())
                .literal_separator(true)
                .build()
                .expect("Exclude glob pattern should be correct"),
        );
    }

    let globs = builder.build().expect("Glob patterns should be correct");
    let exclude_globs = exclude_builder
        .build()
        .expect("Exclude glob patterns should be correct");
    let client = Client::new();

    let regex = Regex::new(
        "https?://(?:[[:alnum:]]+\\.)?[[:alnum:]]+\\.[[:alpha:]]{2,3}/?(?:[[:alnum:]]|[-$_.+!*/&?%=@,:])*",
    )
    .expect("Valid regex");

    for path in cli.path {
        for file in WalkBuilder::new(path)
            .standard_filters(false)
            .hidden(!cli.hidden)
            .build()
            .into_iter()
            .filter_map(Result::ok)
        {
            match file.metadata() {
                Ok(m) => {
                    if !m.is_file() {
                        continue;
                    }
                }
                Err(_) => continue,
            }

            if !globs.is_match(file.path()) {
                continue;
            }

            if exclude_globs.is_match(file.path()) {
                continue;
            }

            println!("{}", file.path().display());

            if cli.list {
                continue;
            }

            let content = match read_to_string(file.path()) {
                Ok(content) => content,
                Err(_) => continue,
            };

            for url in regex.find_iter(&content) {
                if cli
                    .ignore
                    .clone()
                    .into_iter()
                    .any(|ignored| ignored == url.as_str())
                {
                    continue;
                }

                if cli.dry {
                    println!("{}", Blue.paint(url.as_str()));
                    continue;
                }

                let response = match client.get(url.as_str()).send() {
                    Ok(r) => r,
                    Err(_) => {
                        println!(
                            "{} {}",
                            Yellow.paint("Too many redirections for"),
                            Blue.paint(url.as_str())
                        );
                        continue;
                    }
                };

                if response.status().is_success() {
                    println!("URL {} is alive", Blue.paint(url.as_str()));
                } else {
                    println!(
                        "URL {} is NOT alive: {}",
                        Blue.paint(url.as_str()),
                        response.status()
                    );
                    result = ExitCode::FAILURE;
                }
            }
        }
    }

    result
}
