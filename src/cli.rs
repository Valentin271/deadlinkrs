use clap::{arg, command, ArgMatches};

pub struct Cli {
    pub path: Vec<String>,
    pub glob: Vec<String>,
    pub exclude: Vec<String>,
    pub ignore: Vec<String>,
    pub hidden: bool,
    pub list: bool,
    pub dry: bool,
}

impl Cli {
    pub fn new() -> Self {
        let matches: ArgMatches = command!()
            .arg(arg!([path]... "Path to look for files").default_value("."))
            .arg(arg!(-g --glob <glob>... "Unix-style glob to filter files").default_value("**"))
            .arg(arg!(-e --exclude <glob>... "Unix-style glob to exclude from selection"))
            .arg(arg!(--hidden "Includes hidden files and directories"))
            .arg(arg!(-i --ignore <url>... "URL to ignore"))
            .arg(arg!(--list "List searched files and exits"))
            .arg(arg!(--dry "Extract and print URLs that should be requested but don't send requests"))
            .get_matches();

        let mut out = Self {
            path: matches
                .get_many::<String>("path")
                .expect("path arguments should be valid")
                .map(String::from)
                .collect(),
            glob: matches
                .get_many::<String>("glob")
                .expect("glob arguments should be valid")
                .map(String::from)
                .collect(),
            exclude: matches
                .get_many::<String>("exclude")
                .unwrap_or_default()
                .map(String::from)
                .collect(),
            ignore: matches
                .get_many::<String>("ignore")
                .unwrap_or_default()
                .map(String::from)
                .collect(),
            hidden: matches.get_flag("hidden"),
            list: matches.get_flag("list"),
            dry: matches.get_flag("dry"),
        };

        if !out.hidden {
            out.exclude
                .append(&mut vec![String::from("**/.*/**"), String::from("**/.*")]);
        }

        out
    }
}
