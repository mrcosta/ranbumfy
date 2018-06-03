extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};

pub fn get_followed_artists() {
    get_all_followed_artists();
}

fn get_all_followed_artists() {
    let spotify = get_spotify();

    let mut next_request = None;
    loop {
        let response = spotify.current_user_followed_artists(50, next_request);
        let artists = response.ok().unwrap().artists;

        for artist in artists.items {
            println!("{}", artist.name);
        }

        next_request = artists.cursors.after;
        println!("{:?}", next_request);

        if next_request.is_none() { break; }
    }
}

fn get_spotify() -> Spotify {
    let token_info = get_token_info();

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    spotify
}

fn get_token_info() -> TokenInfo {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();

    match get_token(&mut oauth) {
        Some(token_info) => token_info,
        None => panic!("tretas"),
    }
}