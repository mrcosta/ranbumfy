#![feature(custom_attribute)]

mod music_service;
mod spotify_music_service;
mod user;

use env_logger;
use spotify_music_service::authentication::get_spotify_client;
use spotify_music_service::SpotifyClient;
use user::UserService;

fn main() {
    env_logger::init();

    let user_service = UserService {
        music_client: &SpotifyClient {
            client: get_spotify_client(),
        },
    };

    // TODO: draw a recent followed artist to listen to
    // get followed artists that release album in 2019 and it's events
    // implement display for artists
    user_service.draw_albums_to_list();
}
