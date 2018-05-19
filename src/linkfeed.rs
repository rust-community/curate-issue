use linkinfo::LinkInfo;
use rss::{Channel, ChannelBuilder, Item, ItemBuilder, Guid};

pub struct LinkFeed {
    links: Vec<LinkInfo>,
}

fn deduplicate_links(links: &[LinkInfo]) -> Vec<LinkInfo> {
    // TODO: use unique_by from itertools
    let mut seen_urls: Vec<String> = vec![];

    let mut filtered_links: Vec<LinkInfo> = vec![];

    for link in links {
        if !seen_urls.iter().any(|seen| *seen == link.resolved_url) {
            seen_urls.push(link.resolved_url.clone());
            filtered_links.push(link.clone());
        }
    }

    filtered_links
}

#[test]
fn test_deduplicate_links() {
    let link_a = LinkInfo {
        url: "http://t.co/a".to_string(),
        resolved_url: "http://example.com/a.long.html".to_string(),
        title: None,
        publication_date: None,
    };
    let link_b = LinkInfo {
        url: "http://t.co/b".to_string(),
        resolved_url: "http://example.com/a.long.html".to_string(),
        title: None,
        publication_date: None,
    };
    let link_c = LinkInfo {
        url: "http://t.co/c".to_string(),
        resolved_url: "http://example.com/c.long.html".to_string(),
        title: None,
        publication_date: None,
    };
    let link_d = LinkInfo {
        url: "http://t.co/d".to_string(),
        resolved_url: "http://example.com/d.long.html".to_string(),
        title: None,
        publication_date: None,
    };

    let links = vec![link_a, link_b, link_c, link_d];
    let unique_links = deduplicate_links(&links);

    assert_eq!(unique_links.len(), 3);
    assert_eq!(unique_links[0].url, "http://t.co/a");
    assert_eq!(unique_links[1].url, "http://t.co/c");
    assert_eq!(unique_links[2].url, "http://t.co/d");

}

/// A LinkFeed builds an RSS feed from a collection of URLs
impl LinkFeed {
    pub fn new(urls: &[String]) -> LinkFeed {
        let links: Vec<LinkInfo> = urls.iter().map(|url| LinkInfo::from_url(url)).collect();

        LinkFeed { links: deduplicate_links(&links) }
    }
    
    /// Creates an RSS Feed item from a link
    fn build_item(&self, link: &LinkInfo) -> Item {

        // Use the link as title if it cannot be scraped

        let title = match link.title { 
            Some(ref title) => title.to_string(),
            _ => link.url.to_string(),
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
        self.update_items(&[])
    }
    
    /// Adds new items to an existing collection of items
    fn update_items(&self, items: &[Item]) -> Vec<Item> {
        let new_items = self.links
            .iter()
            .filter(|link| {
                !items.iter().any(|item| item.link() == Some(&link.url))
            })
            .map(|link| self.build_item(link));

        let mut combined_items: Vec<Item> = items.to_vec();
        combined_items.extend(new_items);

        combined_items

    }

    /// Creates a Feed bases on either a Channel or a ChannelBuilder
    pub fn build_rss(&self, channel: Option<Channel>, builder: &mut ChannelBuilder) -> Channel {
        match channel {
            Some(mut channel) => {
                let items = self.update_items(channel.items());
                channel.set_items(items);
                channel
            }
            None => builder.items(self.build_items()).build().unwrap(),
        }
    }
}