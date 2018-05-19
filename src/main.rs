//!
//! Curate Issue is a tool to extract links from a GitHub issue, like
//! [these](https://github.com/rust-community/content-o-tron/issues/6) and emit them as
//! RSS feed.
//! It dedupes them, extracts the title as well as the post date from the linked page and
//! extends an already existing RSS file, if present.
//! 
//! # Example
//! ```cargo run rust-community/content-o-tron/issues/6 rss_feed.xml```

extern crate rss;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate scraper;
extern crate hyper;

mod autolink;
mod github;
mod linkinfo;
mod linkfeed;
mod cli;

fn main() {
    cli::cli();
}
