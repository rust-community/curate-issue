extern crate rss;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate scraper;
extern crate hyper;


mod autolink;
mod github;
mod linkinfo;

use github::{GithubIssue};
use linkinfo::{LinkInfo};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder, Guid};
use std::env;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;



const BASE_ISSUE_URL:&'static str = "https://api.github.com/repos";

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
        
        let mut guid = Guid::default();
        guid.set_value(link.resolved_url.as_str());
        guid.set_permalink(true);
        
        ItemBuilder::default()
            .title(title)
            .link(link.url.to_string())
            .guid(guid)
            .build()
            .unwrap()
    }
    
    fn build_items(&self) -> Vec<Item> {
        self.links.iter().map(|link| self.build_item(link)).collect()
    }

    fn update_items(&self, items:&[Item]) -> Vec<Item> {
        let new_items = self.links.iter()
            .filter(|link| !items.iter().any(|item| item.link() == Some(&link.url)))
            .map(|link| self.build_item(link));

        let mut combined_items:Vec<Item> = items.iter().map(|item| item.clone()).collect();
        combined_items.extend(new_items);

        combined_items
        
    }
    
    fn build_rss(&self, channel:Option<Channel>) -> Channel {
        match channel {
            Some(mut channel) => {
                let items = self.update_items(channel.items());
                channel.set_items(items); 
                channel
            }
            None => ChannelBuilder::default()
                .title("Channel Title")
                .link("http://example.com")
                .description("An RSS feed.")
                .items(self.build_items())
                .build()
                .unwrap()
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage {} <github_user/repo/issues/issue_id> <rss file>", &args[0]);
        return;
    }
    
    let url_fragment = &args[1];
    let rss_file_name = &args[2];
    
    let rss_file_path = Path::new(rss_file_name);

    let channel:Option<Channel>;

    if rss_file_path.exists() {
        let f = File::open(rss_file_name).expect("Cannot open file");
        channel = Some(Channel::read_from(BufReader::new(f)).unwrap());
    } else {
        channel = None;
    }

    let issue_url = format!("{}/{}", BASE_ISSUE_URL, url_fragment);
    
    let issue = GithubIssue::get(&issue_url).unwrap().get_comments().unwrap();
    let comments = issue.comments.unwrap();
    
    let mut comment_links = comments.iter().flat_map(|comment| autolink::extract_links(&comment.body)).collect::<Vec<String>>();
    
    let mut links = autolink::extract_links(&issue.body);
    links.append(&mut comment_links);
    
    let link_feed = LinkFeed::new(&links);
    
    let channel = link_feed.build_rss(channel);
    
    let file = File::create(rss_file_name).unwrap();
    channel.write_to(file).unwrap(); // write to the channel to a writer
    
    let string = channel.to_string();
    println!("{}", string)
}
