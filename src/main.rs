mod music_service;
mod spotify_music_service;
mod user;

use env_logger;
use spotify_music_service::authentication::get_spotify_client;
use spotify_music_service::SpotifyClient;
use user::UserService;
use clap::App;

#[tokio::main]
async fn main() {
    // TODO: draw a recent followed artist to listen to
    // get followed artists that release album in 2019 and it's events
    // implement display for artists

    let app = App::new("ranbumfy")
        .version("0.1")
        .author("Mateus Costa <mateuscomp@gmail.com>")
        .args_from_usage("-w, --with-albums 'It draws also album with the artist'");

    let matches = app.get_matches();

    let user_service = init().await;
    if matches.is_present("with-albums") {
        user_service.draw_albums_to_list().await;
    }
}

async fn init() -> UserService {
    env_logger::init();

    UserService {
        music_client: Box::new(SpotifyClient {
            client: get_spotify_client().await,
        }),
    }
}
