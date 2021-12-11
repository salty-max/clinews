use ansi_term::Colour;
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

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        println!("{}", Colour::Green.bold().paint(a.title.to_string()));
        println!("{}", Colour::Yellow.paint(format!("> {}\n", a.url)));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = format!(
        "https://newsapi.org/v2/top-headlines?country=fr&apiKey={}",
        env::var("API_KEY").unwrap()
    );

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
