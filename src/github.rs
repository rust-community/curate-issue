//! A very simple GitHub API module 

use reqwest;
use hyper::header::{Link, RelationType};

#[derive(Deserialize)]
pub struct GithubComment {
    pub body: String,
}




#[derive(Deserialize)]
pub struct GithubIssue {
    comments_url: String,
    pub title: String,
    pub body: String,

    #[serde(skip)]
    pub comments: Option<Vec<GithubComment>>,
}

impl GithubIssue {
    pub fn get(url: &str) -> Result<GithubIssue, reqwest::Error> {
        let mut res = reqwest::get(url)?;
        let issue: GithubIssue = res.json()?;
        Ok(issue)
    }

    pub fn get_comments(mut self: GithubIssue) -> Result<GithubIssue, reqwest::Error> {
        let mut next_page = Some(self.comments_url.to_string());
        let mut comments: Vec<GithubComment> = vec![];

        while let Some(comment_url) = next_page {

            let mut res = reqwest::get(&comment_url)?;
            let mut new_comments: Vec<GithubComment> = res.json()?;

            comments.append(&mut new_comments);

            next_page = res.headers().get::<Link>()
            // Find the next header
            .and_then(|header| 
                    header.values().iter().find(|v| v.rel().unwrap_or(&[]).iter().any(|rel| rel == &RelationType::Next ))
            )
            // Extract the Link
            .and_then(|header| Some(header.link().to_string()));

        }

        self.comments = Some(comments);

        Ok(self)
    }
}

#[test]
fn pagination() {
    let issue = GithubIssue::get(
        "https://api.github.com/repos/fourplusone/curate-issue/issues/1",
    ).unwrap()
        .get_comments()
        .unwrap();
    assert!(issue.comments.unwrap().len() > 30)
}
