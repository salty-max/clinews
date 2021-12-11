use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    dbg!(articles);

    todo!()
}

fn main() {
    dotenv().ok();

    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=fr&apiKey={}",
        env::var("API_KEY").unwrap()
    );

    let articles = get_articles(&url);
}
