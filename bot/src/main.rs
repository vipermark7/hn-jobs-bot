use std::error::Error;
use rss::Channel;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url = "https://hnrss.org/jobs";
    let channel = get_feed(url).await.unwrap();
    // print!("{:?}", channel.items.iter().map(f))
    let title = channel.items[0].title.clone().unwrap();
    let raw_description = channel.items[0].description.clone().unwrap();
    let html = Html::parse_fragment(&raw_description);

    let selector = Selector::parse("a").unwrap();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            println!("Found link: {}", href);
        }
    }
    println!("{:?}", title);
    
    /*
    Example output:
    "Glass Health (YC W23) is hiring founding, senior and lead full-stack engineers"
    "<p>Article URL: <a href=\"https://jobs.lever.co/glass-health-inc?team=Product%20%26%20Engineering\">https://jobs.lever.co/glass-health-inc?team=Product%20%26%20Engineering</a></p>\n
    <p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=41976698\">https://news.ycombinator.com/item?id=41976698</a></p>\n<p>Points: 0</p>\n<p># Comments: 0</p>"
     */
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
