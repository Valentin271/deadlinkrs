extern crate clap;
extern crate globset;
extern crate walkdir;

use clap::{arg, command};
use globset::{GlobBuilder, GlobSetBuilder};
use regex::Regex;
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
                .unwrap(),
        );
    }

    let globs = builder.build().unwrap();

    let regex = Regex::new("(href=[\"'])?(https?://.+)[\"']?").unwrap();
    let mut total = 0;

    for path in matches.get_many::<String>("path").unwrap() {
        for file in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            if !file.metadata().unwrap().is_file() {
                continue;
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
            let count = regex.find_iter(&content).count();

            if count > 0 {
                total += count;
                println!("{} has {} link(s)", file.path().display(), count);
            }
        }
    }

    println!("{} links total", total);
}
