use globset::{Glob, GlobSetBuilder};

use crate::common::TestData;
use deadlinkrs::{Cli, File, Files};

mod common;

#[test]
fn list() {
    let cli = Cli {
        path: vec![String::from("tests_data")],
        list: true,
        ..Cli::default()
    };

    let mut res: Vec<File> = Files::find(&cli).collect();
    res.sort();

    assert_eq!(res, TestData::all());
}

#[test]
fn list_hidden() {
    let cli = Cli {
        path: vec![String::from("tests_data/hidden")],
        hidden: true,
        list: true,
        ..Cli::default()
    };

    let mut res: Vec<File> = Files::find(&cli).collect();
    res.sort();

    assert_eq!(res, TestData::hidden());
}

#[test]
fn list_html() {
    let cli = Cli {
        path: vec![String::from("tests_data")],
        glob: GlobSetBuilder::new()
            .add(Glob::new("**/*.html").unwrap())
            .build()
            .unwrap(),
        list: true,
        ..Cli::default()
    };

    let mut res: Vec<File> = Files::find(&cli).collect();
    res.sort();

    assert_eq!(res, TestData::html());
}

#[test]
fn list_hidden_html() {
    let cli = Cli {
        path: vec![String::from("tests_data")],
        glob: GlobSetBuilder::new()
            .add(Glob::new("**/*.html").unwrap())
            .build()
            .unwrap(),
        hidden: true,
        list: true,
        ..Cli::default()
    };

    let mut res: Vec<File> = Files::find(&cli).collect();
    res.sort();

    assert_eq!(res, TestData::hidden_html());
}
