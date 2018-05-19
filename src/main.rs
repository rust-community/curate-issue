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

use github::GithubIssue;
use linkfeed::LinkFeed;
use rss::{Channel, ChannelBuilder};
use std::env;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;


const BASE_ISSUE_URL: &str = "https://api.github.com/repos";

fn extract_links_from_issue(issue: &mut GithubIssue) -> Option<Vec<String>>{
    issue.load_comments().unwrap();
    let comments = &issue.comments;
    
    if let Some(comments) = comments {
        let mut comment_links = comments
            .iter()
            .flat_map(|comment| autolink::extract_links(&comment.body))
            .collect::<Vec<String>>();

        let mut links = autolink::extract_links(&issue.body);
        links.append(&mut comment_links);
    
        return Some(links);
    }
    
    None
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage {} <github_user/repo/issues/issue_id> [rss file]",
            &args[0]
        );
        return;
    }

    let url_fragment = &args[1];

    let rss_file_name = if args.len() > 2 { Some(&args[2]) } else { None };

    let channel = if let Some(file_name) = rss_file_name {
        let rss_file_path = Path::new(file_name);
        if rss_file_path.exists() {
            let f = File::open(file_name).expect("Cannot open file");
            Some(Channel::read_from(BufReader::new(f)).unwrap())
        } else {
            None
        }
    } else {
        None
    };

    let issue_url = format!("{}/{}", BASE_ISSUE_URL, url_fragment);
    let mut issue = GithubIssue::get(&issue_url).unwrap();
    
    let links = extract_links_from_issue(&mut issue).unwrap();
    
    let link_feed = LinkFeed::new(&links);

    let mut builder = ChannelBuilder::default()
        .title(issue.title)
        .description(issue.body)
        .to_owned();

    let channel = link_feed.build_rss(channel, &mut builder);

    if let Some(file_name) = rss_file_name {
        let file = File::create(file_name).unwrap();
        channel.write_to(file).unwrap(); // write to the channel to a writer
    }

    let string = channel.to_string();
    println!("{}", string)
}
