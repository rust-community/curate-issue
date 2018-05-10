use reqwest;
use scraper::{Html, Selector};

pub struct LinkInfo {
    pub url: String,
    pub title: Option<String>,
    //TODO: use a proper data datatype
    pub publication_date: Option<String>
}

fn fetch_html(url: &str) -> Result<Html, reqwest::Error> {
    let mut res = reqwest::get(url)?;
    let document = Html::parse_document(&res.text()?);
    
    Ok(document)
}


fn get_twitter_title(document: &Html) -> Option<String> {
    
    let selector = Selector::parse("meta[name=twitter\\:title]").unwrap();
    
    for element in document.select(&selector) {
        let title = match element.value().attr("content") {
            Some(txt) => Some(txt.to_string()),
            _ => None
        };
        
        if title.is_some() { return title }
    }
    
    None
}

fn get_og_title(document: &Html) -> Option<String> {
    
    let selector = Selector::parse("meta[property=og\\:title]").unwrap();
    
    for element in document.select(&selector) {
        let title = match element.value().attr("content") {
            Some(txt) => Some(txt.to_string()),
            _ => None
        };
        
        if title.is_some() { return title }
    }
    
    None
}

fn get_html_title(document: &Html) -> Option<String> {
    
    let selector = Selector::parse("title").unwrap();
    
    for element in document.select(&selector) {
        
        let title = match element.text().next() {
            Some(txt) => Some(txt.to_string()),
            _ => None
        };
        return title;
    }
    
    None
}

impl LinkInfo {
    
    pub fn from_url(url: &str) -> LinkInfo {
        let document = fetch_html(url).unwrap();
        
        let mut title;
        
        title = get_twitter_title(&document);
        
        if title.is_none() {
            title = get_og_title(&document);
        }
        
        if title.is_none() {
            title = get_html_title(&document);
        }
        
        LinkInfo {url: url.to_string(), title:title, publication_date:None}
    }
}


fn test_title(url: &str, expected: &str) {
    let link_info = LinkInfo::from_url(url);
    assert_eq!(link_info.url, url);
    assert_eq!(link_info.title, Some(expected.to_string()));
}

#[test]
fn test_all_hands() {
    test_title("https://blog.rust-lang.org/2018/04/06/all-hands.html", 
        "The Rust Team All Hands in Berlin: a Recap");
    
}

#[test]
fn test_sound_and_ergonomic() {
    test_title(
        "https://aturon.github.io/2018/04/05/sound-specialization/",
        "Sound and ergonomic specialization for Rust")
}
