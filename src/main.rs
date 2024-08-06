mod builder;
mod cli;

use builder::anime_theme::AnimeGenreBuilder;
use cli::cli;

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("get-anime-themes", _args)) => _get_anime_themes().await,

        Some((ext, _)) => {
            println!("Unknown command {}", ext);
        }
        _ => unreachable!(),
    }
}

async fn _get_anime_themes() {
    let dictionary = AnimeGenreBuilder::new().build_dictionary().await.unwrap();
    for items in dictionary {
        println!("{}", items)
    }
}
