#![feature(custom_attribute)]

mod music_service;
mod spotify_music_service;
mod user;

use spotify_music_service::SpotifyClient;
use spotify_music_service::authentication::get_spotify_client;
use user::draw_an_album_to_list;

fn main() {
    let spotify_client = SpotifyClient {
        client: get_spotify_client(),
    };
    draw_an_album_to_list(&spotify_client);
}
