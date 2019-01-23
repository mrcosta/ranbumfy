mod music_service;
mod spotify;
mod user;

use spotify::SpotifyClient;
use user::draw_an_album_to_list;

fn main() {
    let spotify_client = SpotifyClient {};
    draw_an_album_to_list(&spotify_client);
}
