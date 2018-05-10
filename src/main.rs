extern crate rss;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate scraper;

mod autolink;
mod github;
mod linkinfo;

use github::{GithubIssue};

use linkinfo::{LinkInfo};

use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

const ISSUE_URL:&'static str = "https://api.github.com/repos/rust-community/content-o-tron/issues/6";

struct LinkFeed {
    links: Vec<LinkInfo>
}

impl LinkFeed {
    fn new(links : &Vec<String>) -> LinkFeed {
        LinkFeed { links: links.iter().map(|link| LinkInfo::from_url(link)).collect() }
    }
    
    fn build_item(&self, link: &LinkInfo) -> Item {
        
        // Use the link as title if it cannot be scraped
        
        let title = match &link.title { 
            &Some(ref title) => title.to_string(),
            _ => link.url.to_string()
        };
        
        ItemBuilder::default()
            .title(title)
            .link(link.url.to_string())
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
