use std::error::Error;
use rss::{Channel, Item};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url = "https://hnrss.org/jobs";
    let channel = get_feed(url).await.unwrap();
    let jobs = parse_items(channel.items());

    dbg!(jobs);
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}


#[derive(Debug)]
struct Job {
    title: String,
    article_url: String,
    comments_url: Result<String, String>
}

fn parse_items(items: &[Item]) -> Vec<Job> {
    let mut jobs = Vec::new();
    items.iter().for_each(|item| {
        let raw_description = item.description().unwrap();
        let html = Html::parse_fragment(&raw_description);
        let selector: Selector = Selector::parse("a").unwrap();

        let urls = html
            .select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .collect::<Vec<_>>();

        let title = item.title().unwrap().to_string();
        let article_url = urls[0].to_string();
        let comments_url = urls.get(1)
            .ok_or("no comments url");
        
        jobs.push(Job {
            title,
            article_url,
            comments_url
        });
    });
    jobs
}