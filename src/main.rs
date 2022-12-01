use std::fs::read_to_string;

use globset::{GlobBuilder, GlobSetBuilder};
use regex::Regex;
use reqwest::blocking::Client;
use walkdir::WalkDir;

use cli::Cli;

mod cli;

fn main() {
    let cli = Cli::new();

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
        for file in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            match file.metadata() {
                Ok(m) => {
                    if !m.is_file() {
                        continue;
                    }
                }
                Err(_) => continue,
            }

            let filepath = if file.path().starts_with("./") {
                file.path()
                    .strip_prefix("./")
                    .expect("./ prefix should be stripped")
            } else if file.path().starts_with("../") {
                file.path()
                    .strip_prefix("../")
                    .expect("../ prefix should be stripped")
            } else {
                file.path()
            };

            if !globs.is_match(filepath) {
                continue;
            }

            if exclude_globs.is_match(filepath) {
                continue;
            }

            if cli.list {
                println!("{}", file.path().display());
                continue;
            }

            let content = match read_to_string(file.path()) {
                Ok(content) => content,
                Err(_) => continue,
            };

            println!("{}", file.path().display());

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
                    println!("{}", url.as_str());
                    continue;
                }

                let response = client
                    .get(url.as_str())
                    .send()
                    .expect("Request should get response");

                if response.status().is_success() {
                    println!("URL {} is alive", url.as_str())
                } else {
                    println!("URL {} is NOT alive: {}", url.as_str(), response.status());
                }
            }
        }
    }
}
