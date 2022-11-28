extern crate clap;
extern crate globset;
extern crate walkdir;

use clap::{arg, command};
use globset::{GlobBuilder, GlobSetBuilder};
use walkdir::WalkDir;

fn main() {
    let matches = command!()
        .arg(arg!([path]... "Path to look for files").default_value("."))
        .arg(arg!(-g --glob <glob>... "Unix-style glob to filter files").default_value("**"))
        .arg(arg!(--hidden "Includes hidden files and directories"))
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
            // let _content = read_to_string(file.path()).expect("File could not be red");
        }
    }
}
