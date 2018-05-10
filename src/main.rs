extern crate reqwest;
extern crate pulldown_cmark;
extern crate rss;
extern crate regex;
mod autolink;

#[macro_use]
extern crate serde_derive;

use pulldown_cmark::{Parser, Event, Tag};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

const ISSUE_URL:&'static str = "https://api.github.com/repos/rust-community/content-o-tron/issues/6";

#[derive(Deserialize)]
struct GithubComment {
    body: String
}

#[derive(Deserialize)]
struct GithubIssue {
    comments_url: String,
    title: String,
    body: String,
    
    #[serde(skip)]
    comments: Option<Vec<GithubComment>>
}

impl GithubIssue {
    fn get(url: &str) -> Result<GithubIssue, reqwest::Error> {
        let mut res = reqwest::get(url)?;
        let issue: GithubIssue = res.json()?;
        Ok(issue)
    }
    
    fn get_comments(mut self : GithubIssue) -> Result<GithubIssue, reqwest::Error> {
        // TODO: support pagination
        let mut res = reqwest::get(&self.comments_url)?;
        self.comments = res.json()?;
        Ok(self)
    }
}

/// Extracts Links from a markdown String
fn extract_links (markdown : &str) -> Vec<String> {
    
    Parser::new(markdown).filter_map(|event| match event {
        Event::Start(tag) => match tag {
            Tag::Link(url, title) => Some(vec![url.into_owned()]),
            _ => None
        },
        Event::Text(text) => Some(autolink::autolink(&text.into_owned())),
        _ => None
    }).flat_map(|e| e).collect()
}

#[test]
fn test_md_link() {
    assert_eq!(extract_links("[Test](https://google.com)"),vec!["https://google.com"]);
}

#[test]
fn test_auto_link() {
    assert_eq!(extract_links("Hello World https://google.com"),vec!["https://google.com"]);
}

#[test]
fn test_mixed_links() {
    assert_eq!(extract_links("Hello World https://google.com [Test](https://google.de)"),vec!["https://google.com", "https://google.de"]);
}

struct LinkFeed<'a> {
    links: &'a Vec<String>
}

impl <'a> LinkFeed<'a> {
    fn new(links : &Vec<String>) -> LinkFeed {
        LinkFeed { links: links }
    }
    
    fn build_item(&self, link: &str) -> Item {
        ItemBuilder::default()
            .title(link.to_string())
            .link(link.to_string())
            .build()
            .unwrap()
    }
    
    fn build_items(&self) -> Vec<Item> {
        self.links.iter().map(|link| self.build_item(link)).collect()
    }
    
    fn build_rss(&self) -> Channel {
        ChannelBuilder::default()
            .title("Channel Title")
            .link("http://example.com")
            .description("An RSS feed.")
            .items(self.build_items())
            .build()
            .unwrap()
    }
}

fn main() {
    let issue = GithubIssue::get(ISSUE_URL).unwrap().get_comments().unwrap();
    let comments = issue.comments.unwrap();
    
    let mut comment_links = comments.iter().flat_map(|comment| extract_links(&comment.body)).collect::<Vec<String>>();

    let mut links = extract_links(&issue.body);
    
    for x in links.iter() {
        println!("{}", x);
    }
    
    links.append(&mut comment_links);
    
    
    let mut link_feed = LinkFeed::new(&links);
    
    let channel = link_feed.build_rss();
    channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
    let string = channel.to_string();
    println!("{}", string)
}
