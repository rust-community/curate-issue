
extern crate pulldown_cmark;
extern crate regex;

use self::pulldown_cmark::{Parser, Event, Tag};
use self::regex::Regex;

pub fn autolink(text: &str) -> Vec<String> {
    if text.len() == 0 {
        return vec![]
    }

    let re = Regex::new(r"(?ix)
        (?: ((?:ed2k|ftp|http|https|irc|mailto|news|gopher|nntp|telnet|webcal|xmpp|callto|feed|svn|urn|aim|rsync|tag|ssh|sftp|rtsp|afs|file):)// | www\. )
        [^\s<\x{00A0}\x{0022}]+
    ").unwrap();

    re.captures_iter(text).map(|captures| captures[0].to_string()).collect()
}

/// Extracts Links from a markdown String
pub fn extract_links (markdown : &str) -> Vec<String> {

    Parser::new(markdown).filter_map(|event| match event {
        Event::Start(tag) => match tag {
            Tag::Link(url, _title) => Some(vec![url.into_owned()]),
            _ => None
        },
        Event::Text(text) => Some(autolink(&text.into_owned())),
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



