extern crate regex;
extern crate chrono;

use reqwest;
use scraper::{Html, Selector};
use self::chrono::{Utc, TimeZone, DateTime};

use self::regex::Regex;


#[derive(Clone, Debug)]
pub struct LinkInfo {
    pub url: String,
    pub resolved_url: String,
    pub title: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
}

fn get_twitter_title(document: &Html) -> Option<String> {

    let selector = Selector::parse("meta[name=twitter\\:title]").unwrap();

    for element in document.select(&selector) {
        let title = match element.value().attr("content") {
            Some(txt) => Some(txt.to_string()),
            _ => None,
        };

        if title.is_some() {
            return title;
        }
    }

    None
}

fn get_og_title(document: &Html) -> Option<String> {

    let selector = Selector::parse("meta[property=og\\:title]").unwrap();

    for element in document.select(&selector) {
        let title = match element.value().attr("content") {
            Some(txt) => Some(txt.to_string()),
            _ => None,
        };

        if title.is_some() {
            return title;
        }
    }

    None
}

fn get_html_title(document: &Html) -> Option<String> {

    let selector = Selector::parse("title").unwrap();
    document
        .select(&selector)
        .next()
        .and_then(|element| element.text().next())
        .and_then(|text| Some(text.to_string()))
}

/// parses tags like <meta property="article:published_time" content="2018-04-05T00:00:00+00:00">
fn get_document_date(document: &Html) -> Option<DateTime<Utc>> {
    let selector = Selector::parse("meta[property=article\\:published_time]").unwrap();

    document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .and_then(|text| text.parse::<DateTime<Utc>>().ok())
}

/// parses dates in the url like /2018/04/06/
fn get_url_date(url: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(r"/(\d\d\d\d)/(\d\d)/(\d\d)/").unwrap();
    re.captures_iter(url)
        .next()
        .and_then(|capture| {
            Some((
                capture[1].parse::<i32>().unwrap(),
                capture[2].parse::<u32>().unwrap(),
                capture[3].parse::<u32>().unwrap(),
            ))
        })
        .and_then(|(y, m, d)| Some(Utc.ymd(y, m, d).and_hms(0, 0, 0)))
}

impl LinkInfo {
    pub fn from_url(url: &str) -> Result<LinkInfo, reqwest::Error> {
        let mut res = reqwest::get(url)?;

        let document = Html::parse_document(&res.text()?);

        let mut title;

        title = get_twitter_title(&document);

        if title.is_none() {
            title = get_og_title(&document);
        }

        if title.is_none() {
            title = get_html_title(&document);
        }

        let mut publication_date = get_document_date(&document);

        if publication_date.is_none() {
            publication_date = get_url_date(&res.url().to_string());
        }

        Ok(LinkInfo {
            url: url.to_string(),
            resolved_url: res.url().to_string(),
            title,
            publication_date,
        })
    }
}

#[cfg(test)]
fn test_title(url: &str, expected: &str) {
    let link_info = LinkInfo::from_url(url).unwrap();
    assert_eq!(link_info.url, url);
    assert_eq!(link_info.title, Some(expected.to_string()));
}

#[test]
fn test_all_hands() {
    test_title(
        "https://blog.rust-lang.org/2018/04/06/all-hands.html",
        "The Rust Team All Hands in Berlin: a Recap",
    );

}

#[test]
fn test_sound_and_ergonomic() {
    test_title(
        "https://aturon.github.io/2018/04/05/sound-specialization/",
        "Sound and ergonomic specialization for Rust",
    )
}

#[test]
fn test_date_parse() {
    let date = get_url_date("https://aturon.github.io/2018/04/05/sound-specialization/");
    assert_eq!(date.unwrap().to_rfc3339(), "2018-04-05T00:00:00+00:00")
}

#[test]
fn test_date_parse_none() {
    let date = get_url_date("https://google.com/");
    assert_eq!(date, None)
}
