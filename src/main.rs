#![feature(custom_attribute)]

mod music_service;
mod spotify_music_service;
mod user;

use env_logger;
use std::io;
use spotify_music_service::SpotifyClient;
use spotify_music_service::authentication::get_spotify_client;
use user::draw_an_album_to_list;

fn main() {
    env_logger::init();

    let spotify_client = SpotifyClient {
        client: get_spotify_client(),
    };

    // draw 7 albums at once
    // get followed artists that release album in 2019 and it's events
    // implement display for artists
    loop {
        draw_an_album_to_list(&spotify_client);

        println!("Please press any key to draw one more round or `q` to exit");

        let mut continue_drawing = String::new();
        match io::stdin().read_line(&mut continue_drawing) {
            Ok(_line) => (),
            Err(error) => println!("error: {}", error),
        }

        if continue_drawing.trim() == "q" {
            break;
        }
    }
}
