use std::path::Path;

use deadlinkrs::File;

pub struct TestData;

impl TestData {
    fn build(vec: Vec<&str>) -> Vec<File> {
        vec.iter().map(|x| File::new(Path::new(x))).collect()
    }

    /// Every file not hidden
    pub fn all() -> Vec<File> {
        Self::build(vec!["tests_data/README.md", "tests_data/index.html"])
    }

    /// Every file ending in html
    pub fn hidden_html() -> Vec<File> {
        Self::build(vec![
            "tests_data/hidden/.hidden_dir/visible_in_hidden.html",
            "tests_data/index.html",
        ])
    }

    /// Every file ending in html (hidden included)
    pub fn html() -> Vec<File> {
        vec![File::new(Path::new("tests_data/index.html"))]
    }

    /// Every hidden file
    pub fn hidden() -> Vec<File> {
        Self::build(vec![
            "tests_data/hidden/.hidden_dir/visible_in_hidden.html",
            "tests_data/hidden/.hidden_file",
        ])
    }
}
