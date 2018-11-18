extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth, TokenInfo};
use rspotify::spotify::senum::AlbumType;
use rspotify::spotify::util::get_token;
use std::collections::HashMap;

pub fn get_followed_artists() {
    let spotify = get_spotify();
    let artists_and_ids = get_all_followed_artists(&spotify);
    let artist_albums_ids = get_artist_albums_ids(&spotify, artists_and_ids);

    //        for album in &albums {
    //            println!("{:?}", album.artists);
    //        };
    //        let albums_id = albums
    //            .iter()
    //            .map(|album| &album.id)
    //            .collect::<Vec<&String>>();
    //        println!("oie {}", albums_id[0]);

    //        for album_id in album_ids {
    //            let response = spotify.albums(&album_id.id); // use albums
    //            let album = response.ok().unwrap().name;
    //            println!("{}", album);
    //        }
}

fn get_all_followed_artists(spotify: &Spotify) -> HashMap<String, String> {
    let mut next_request = None;
    let mut artists_and_ids = HashMap::new();

    loop {
        let response = spotify.current_user_followed_artists(50, next_request);
        let artists = response.ok().unwrap().artists;

        for artist in artists.items {
            artists_and_ids.insert(artist.name, artist.id);
        }

        next_request = artists.cursors.after;
        if next_request.is_none() {
            break;
        } else {
            println!("Doing next request: {:?}", next_request);
        }
    }

    artists_and_ids
}

fn get_artist_albums_ids(
    spotify: &Spotify,
    artists_and_ids: HashMap<String, String>,
) -> HashMap<String, Vec<String>> {
    let mut artist_and_albums_ids = HashMap::new();

    println!("Number of artists: {}", artists_and_ids.iter().len());
    for (artist, id) in artists_and_ids {
        let response = spotify.artist_albums(&id, Some(AlbumType::Album), None, Some(50), None);
        let albums = response.ok().unwrap().items;

        let albums_ids = albums
            .into_iter()
            .map(|album| album.id)
            .collect::<Vec<String>>();

        println!("Got the follow albums from {}: {:?}", artist, albums_ids);
        artist_and_albums_ids.insert(artist, albums_ids);
    }

    artist_and_albums_ids
}

fn get_spotify() -> Spotify {
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}
