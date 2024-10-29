use std::error::Error;
use rss::Channel;

#[tokio::main]
async fn main() {
    let url = "https://hnrss.org/jobs";
    let channel = get_feed(url).await.unwrap();
    print!("{}", channel.to_string())
}

async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
