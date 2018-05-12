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


const BASE_ISSUE_URL:&str = "https://api.github.com/repos";

struct LinkFeed {
    links: Vec<LinkInfo>
}

fn deduplicate_links(links: &[LinkInfo]) -> Vec<LinkInfo> {
    // TODO: use unique_by from itertools
    let mut seen_urls:Vec<String> = vec![];

    let mut filtered_links:Vec<LinkInfo> = vec![];
    
    for link in links {
        if !seen_urls.iter().any(|seen| *seen == link.resolved_url) { 
            seen_urls.push(link.resolved_url.clone());
            filtered_links.push(link.clone());
        }
    }
    
    filtered_links
}

#[test]
fn test_deduplicate_links (){
    let link_a = LinkInfo {url: "http://t.co/a".to_string(),  resolved_url: "http://example.com/a.long.html".to_string(), title: None, publication_date: None };
    let link_b = LinkInfo {url: "http://t.co/b".to_string(),  resolved_url: "http://example.com/a.long.html".to_string(), title: None, publication_date: None };
    let link_c = LinkInfo {url: "http://t.co/c".to_string(),  resolved_url: "http://example.com/c.long.html".to_string(), title: None, publication_date: None };
    let link_d = LinkInfo {url: "http://t.co/d".to_string(),  resolved_url: "http://example.com/d.long.html".to_string(), title: None, publication_date: None };
    
    let links = vec!{link_a, link_b, link_c, link_d};
    let unique_links = deduplicate_links(&links);
    
    assert_eq!(unique_links.len(), 3);
    assert_eq!(unique_links[0].url, "http://t.co/a");
    assert_eq!(unique_links[1].url, "http://t.co/c");
    assert_eq!(unique_links[2].url, "http://t.co/d");
    
}



impl LinkFeed {
    fn new(urls : &[String]) -> LinkFeed {
        let links : Vec<LinkInfo>  = urls.iter()
            .map(|url| LinkInfo::from_url(url))
            .collect();
            
        LinkFeed { links: deduplicate_links(&links) }
    }
    
    fn build_item(&self, link: &LinkInfo) -> Item {
        
        // Use the link as title if it cannot be scraped
        
        let title = match link.title { 
            Some(ref title) => title.to_string(),
            _ => link.url.to_string()
        };
        
        let mut guid = Guid::default();
        guid.set_value(link.resolved_url.as_str());
        guid.set_permalink(true);
        
        ItemBuilder::default()
            .title(title)
            .link(link.url.to_string())
            .pub_date(link.publication_date.and_then(|d| Some(d.to_rfc2822())))
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

        let mut combined_items:Vec<Item> = items.to_vec();
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
    
    if args.len() < 2 {
        println!("Usage {} <github_user/repo/issues/issue_id> [rss file]", &args[0]);
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
    
    let issue = GithubIssue::get(&issue_url).unwrap().get_comments().unwrap();
    let comments = issue.comments.unwrap();
    
    let mut comment_links = comments.iter().flat_map(|comment| autolink::extract_links(&comment.body)).collect::<Vec<String>>();
    
    let mut links = autolink::extract_links(&issue.body);
    links.append(&mut comment_links);
    
    let link_feed = LinkFeed::new(&links);
    
    let channel = link_feed.build_rss(channel);
    if let Some(file_name) = rss_file_name {
        let file = File::create(file_name).unwrap();
        channel.write_to(file).unwrap(); // write to the channel to a writer
    }

    let string = channel.to_string();
    println!("{}", string)
}
