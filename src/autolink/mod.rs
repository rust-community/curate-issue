

use regex::Regex;

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


