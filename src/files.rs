use ignore::WalkBuilder;

use crate::cli::Cli;
use crate::files::file::File;
use crate::files::links::cache::Cache;
use crate::files::links::results::Results;

mod file;
pub mod links;

/// Represents a list of files
pub struct Files {
    cache: Cache,
}

impl Files {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }

    /// Find files matching the globs
    fn find<'a, 'b>(cli: &'a Cli) -> impl Iterator<Item = File> + 'b
    where
        'a: 'b,
    {
        let mut builder = WalkBuilder::new(cli.path.first().unwrap());

        for path in &cli.path[1..] {
            builder.add(path);
        }

        builder
            .standard_filters(false)
            .hidden(!cli.hidden)
            .build()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|x| x.metadata().unwrap().is_file())
            .filter(|x| cli.glob.is_match(x.path()))
            .filter(|x| !cli.exclude.is_match(x.path()))
            .map(|x| File::new(x.path()))
    }

    pub fn check(&mut self, cli: &Cli) -> Results {
        let mut results = Results::new();

        for file in Files::find(cli) {
            results.merge(file.check(cli, &mut self.cache));
        }

        results
    }

    pub fn list(cli: &Cli) {
        for file in Files::find(cli) {
            println!("{}", file);
        }
    }

    pub fn dry(cli: &Cli) {
        for file in Files::find(cli) {
            println!("{}", file);
            file.print_links(cli);
        }
    }
}
