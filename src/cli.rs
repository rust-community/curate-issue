extern crate clap; 
use self::clap::{Arg, App};

use github::GithubIssue;
use linkfeed::LinkFeed;
use autolink;
use rss::{Channel, ChannelBuilder};

use std::fs::File;
use std::path::Path;
use std::io::BufReader;

/// Loads an issue and all its comments from Github, extracts all links, using the autolink module.
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

const BASE_ISSUE_URL: &str = "https://api.github.com/repos";

pub fn cli() {
    let matches = App::new("Curate Issue")
        .version("0.1.0")
        .arg(Arg::with_name("ISSUE")
            .help("The Github issue to take the links from. Example: rust-community/content-o-tron/issues/6")
            .required(true)
            .index(1))
        .arg(Arg::with_name("RSS File")
            .help("The RSS file to be written")
            .index(2))
        .get_matches();
        
    let url_fragment = matches.value_of("ISSUE").unwrap();
    let rss_file_name = matches.value_of("RSS File");
    
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
    let mut issue = GithubIssue::get(&issue_url).expect("Error fetching Github Issue");
    
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
