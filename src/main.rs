use clap::{arg, command};
use globset::{GlobBuilder, GlobSetBuilder};
use regex::Regex;
use reqwest::blocking::Client;
use std::fs::read_to_string;
use walkdir::WalkDir;

fn main() {
    let matches = command!()
        .arg(arg!([path]... "Path to look for files").default_value("."))
        .arg(arg!(-g --glob <glob>... "Unix-style glob to filter files").default_value("**"))
        // .arg(arg!(--exclude <glob>... "Unix-style glob to exclude from selection"))
        // .arg(arg!(--hidden "Includes hidden files and directories"))
        // .arg(arg!(--ignore <url>... "URL to ignore"))
        .arg(arg!(--list "List searched files and exits"))
        .get_matches();

    let mut builder = GlobSetBuilder::new();

    for glob in matches.get_many::<String>("glob").unwrap_or_default() {
        builder.add(
            GlobBuilder::new(glob)
                .literal_separator(true)
                .build()
                .expect("Glob pattern should be correct"),
        );
    }

    let globs = builder.build().expect("Glob patterns should be correct");
    let client = Client::new();

    let regex = Regex::new(
        "https?://(?:[[:alnum:]]+\\.)?[[:alnum:]]+\\.[[:alpha:]]{2,3}/?(?:[[:alnum:]]|[-$_.+!*/&?%=@,:])*",
    )
    .expect("Valid regex");

    for path in matches.get_many::<String>("path").unwrap_or_default() {
        for file in WalkDir::new(path).into_iter().filter_map(Result::ok) {
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

            if matches.get_flag("list") {
                println!("{}", file.path().display());
                continue;
            }

            let content = match read_to_string(file.path()) {
                Ok(content) => content,
                Err(_) => continue,
            };

            println!("{}", file.path().display());

            for url in regex.find_iter(&content) {
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
