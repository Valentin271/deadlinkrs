use ansi_term::Color::{Blue, Green, Red, Yellow};
use ansi_term::Style;

pub struct Link {
    link: String,
}

impl Link {
    pub fn new(link: &str) -> Self {
        Self {
            link: link.to_string(),
        }
    }

    pub fn print(&self) {
        println!("\t{}", Blue.underline().paint(&self.link));
    }

    pub fn ok(&self, reason: &str) {
        println!(
            "\t{} {} {}",
            Green.paint("[OK]"),
            Blue.underline().paint(&self.link),
            reason
        )
    }

    pub fn warn(&self, reason: &str) {
        println!(
            "\t{} {} {}",
            Yellow.paint("[WARN]"),
            Blue.underline().paint(&self.link),
            Yellow.paint(reason)
        )
    }

    pub fn err(&self, reason: &str) {
        println!(
            "\t{} {} {}",
            Red.paint("[ERR]"),
            Blue.underline().paint(&self.link),
            Red.paint(reason)
        )
    }

    pub fn cache(&self, reason: &str) {
        println!(
            "\t{} {} {}",
            Green.dimmed().paint("[CACHE]"),
            Blue.dimmed().underline().paint(&self.link),
            Style::new().dimmed().paint(reason)
        )
    }
}
