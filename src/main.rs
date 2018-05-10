extern crate rss;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

mod autolink;
mod github;

use github::{GithubIssue};


use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

const ISSUE_URL:&'static str = "https://api.github.com/repos/rust-community/content-o-tron/issues/6";

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
    
    let mut comment_links = comments.iter().flat_map(|comment| autolink::extract_links(&comment.body)).collect::<Vec<String>>();

    let mut links = autolink::extract_links(&issue.body);
    
    for x in links.iter() {
        println!("{}", x);
    }
    
    links.append(&mut comment_links);
    
    
    let link_feed = LinkFeed::new(&links);
    
    let channel = link_feed.build_rss();
    channel.write_to(::std::io::sink()).unwrap(); // write to the channel to a writer
    let string = channel.to_string();
    println!("{}", string)
}
