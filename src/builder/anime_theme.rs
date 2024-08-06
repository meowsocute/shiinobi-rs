use std::arch::x86_64;

use reqwest::Error;
use scraper::{selectable::Selectable, Html, Selector};

struct Dictionary {}

pub struct AnimeGenreBuilder {}

impl AnimeGenreBuilder {
    pub fn new() -> Self {
        Self {}
    }

    fn _build_urls(self, document: Html) -> Vec<String> {
        let a_selector = Selector::parse(r#"a[href*="genre"]"#).unwrap();
        let a_items = document.select(&a_selector);
        a_items
            .map(move |item| {
                "https://myanimelist.net".to_owned() + item.value().attr("href").unwrap()
            })
            .collect()
    }
    pub async fn build_dictionary(self) -> Result<Vec<String>, Error> {
        let response = reqwest::get("https://myanimelist.net/anime.php")
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&response);
        let urls = self._build_urls(document);
        Ok(urls)
    }
}
