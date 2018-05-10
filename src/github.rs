use reqwest;


#[derive(Deserialize)]
pub struct GithubComment {
    pub body: String
}

#[derive(Deserialize)]
pub struct GithubIssue {
    comments_url: String,
    pub title: String,
    pub body: String,
    
    #[serde(skip)]
    pub comments: Option<Vec<GithubComment>>
}

impl GithubIssue {
    pub fn get(url: &str) -> Result<GithubIssue, reqwest::Error> {
        let mut res = reqwest::get(url)?;
        let issue: GithubIssue = res.json()?;
        Ok(issue)
    }
    
    pub fn get_comments(mut self : GithubIssue) -> Result<GithubIssue, reqwest::Error> {
        // TODO: support pagination
        let mut res = reqwest::get(&self.comments_url)?;
        self.comments = res.json()?;
        Ok(self)
    }
}
