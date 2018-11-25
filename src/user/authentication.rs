extern crate rspotify;

use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::spotify::util::get_token;
use rspotify::spotify::client::Spotify;

pub fn get_spotify() -> Spotify {
    let token_info = get_token_info();

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    Spotify::default()
        .client_credentials_manager(client_credential)
        .build()
}

fn get_token_info() -> TokenInfo {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();

    match get_token(&mut oauth) {
        Some(token_info) => token_info,
        None => panic!("tretas"),
    }
}
