use ansi_term::Colour;
use dotenv::dotenv;
use std::env;
use std::error::Error;

use newsapi::{get_articles, Articles};

pub fn render_articles(articles: &Articles) {
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
