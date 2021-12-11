use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to fetch articles")]
    RequestFailed(ureq::Error),
    #[error("Failed to parse response into string")]
    ParseStringFailed(std::io::Error),
    #[error("Failed to parse JSON")]
    JSONParseFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(NewsApiError::RequestFailed)?
        .into_string()
        .map_err(NewsApiError::ParseStringFailed)?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(NewsApiError::JSONParseFailed)?;

    Ok(articles)
}
