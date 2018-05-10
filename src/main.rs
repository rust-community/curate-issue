extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate pulldown_cmark;
use pulldown_cmark::{Parser, Event};

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
        let mut res = reqwest::get(&self.comments_url)?;
        self.comments = res.json()?;
        Ok(self)
    }
}

fn extract_links (markdown : &str) -> Vec<String> {
    
    Parser::new(markdown).filter_map(|event| match event {
    	Event::Text(text) => Some(text.into_owned()),
    	_ => None
    }).collect()
}

fn main() {
    let issue = GithubIssue::get(ISSUE_URL).unwrap().get_comments().unwrap();
    println!("{}", issue.title);
    
    for link in extract_links(&issue.body) {
        println!("{}", link);
        
    }
    
    for comment in issue.comments.unwrap() {
        println!("=======\nComment: \n{}\n=======\nLinks:", comment.body);
        
        for link in extract_links(&comment.body) {
            println!("{}", link);
            
        }
        
        
        
    }
    
}
