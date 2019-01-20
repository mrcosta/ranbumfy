extern crate rand;
extern crate rspotify;

mod user;
mod spotify;
mod music_service;

use user::draw_an_album_to_list;
use spotify::SpotifyClient;

fn main() {
    let spotify_client = SpotifyClient {};
    draw_an_album_to_list(&spotify_client);
}
