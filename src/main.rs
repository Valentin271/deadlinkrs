use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::ExitCode;

use ansi_term::Color::{Green, Red};
use ansi_term::Style;
use globset::{GlobBuilder, GlobSetBuilder};
use human_panic::setup_panic;
use ignore::WalkBuilder;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::StatusCode;

use cli::Cli;

use crate::log::Link;

mod cli;
mod log;

fn main() -> ExitCode {
    setup_panic!();

    let cli = Cli::new();
    let mut cache: HashMap<String, StatusCode> = HashMap::new();
    let mut dead_links: u32 = 0;

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

            if cli.list {
                println!("{}", file.path().display());
                continue;
            }

            println!(
                "{}",
                Style::new()
                    .dimmed()
                    .paint(file.path().display().to_string())
            );

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

                let link = Link::new(url.as_str());

                if cli.dry {
                    link.print();
                    continue;
                }

                if cache.contains_key(url.as_str()) {
                    link.cache("");
                    continue;
                }

                let response = match client.get(url.as_str()).send() {
                    Ok(r) => r,
                    Err(_) => {
                        link.warn("Too many redirections");
                        continue;
                    }
                };

                cache.insert(String::from(url.as_str()), response.status());

                if response.status().is_success() {
                    link.ok("");
                } else {
                    link.err(format!("{}", response.status()).as_str());
                    dead_links += 1;
                }
            }
        }
    }

    println!();
    if dead_links > 0 {
        println!("{}", Red.paint(format!("Found {} dead links", dead_links)));
        ExitCode::FAILURE
    } else {
        println!("{}", Green.paint("No dead links !"));
        ExitCode::SUCCESS
    }
}
