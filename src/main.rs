#![feature(custom_attribute)]

mod music_service;
mod spotify_music_service;
mod user;

use spotify_music_service::SpotifyClient;
use user::draw_an_album_to_list;

fn main() {
    let spotify_client = SpotifyClient {};
    draw_an_album_to_list(&spotify_client);
}
