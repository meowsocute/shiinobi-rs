mod builder;
use builder::anime_theme::AnimeGenreBuilder;
// use futures::executor::block_on;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    _get_anime_themes().await;
    // block_on(anime)
}

async fn _get_anime_themes() {
    let dictionary = AnimeGenreBuilder::new().build_dictionary().await.unwrap();
    for items in dictionary {
        println!("{}", items)
    }
    // println!("{:?}", dictionary)
}
