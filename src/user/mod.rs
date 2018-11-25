mod authentication;
mod artist;

use user::authentication::get_spotify;
use user::artist::{ Album, Artist };
use rand::thread_rng;
use rand::seq::SliceRandom;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::senum::AlbumType;
use std::collections::HashMap;

pub fn get_followed_artists() {
    let spotify = get_spotify();
    let artists_and_ids = get_all_followed_artists(&spotify);
    let artists_names = artists_and_ids
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    let randomized_artist_name = artists_names.choose(& mut thread_rng()).unwrap();
    let artist_id = &artists_and_ids[randomized_artist_name];
    let randomized_album = get_randomized_album(&spotify, &artist_id);

    println!("You are going to listen to {} from {}", randomized_album.name, randomized_artist_name);
    println!("Here's the url: {}", randomized_album.url);
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
            // TODO: use log info
//            println!("Doing next request: {:?}", next_request);
        }
    }

    artists_and_ids
}

fn get_randomized_album(spotify: &Spotify, id: &str) -> Album {
    let response = spotify.artist_albums(&id, Some(AlbumType::Album), None, Some(50), None);
    let albums = response.ok().unwrap().items;

    let randomized_album_from_response = albums.choose(& mut thread_rng()).unwrap();
    let name = &randomized_album_from_response.name;
    let id = &randomized_album_from_response.id;
    let url = &randomized_album_from_response.external_urls["spotify"];

    Album {
        name: name.to_string(),
        id: id.to_string(),
        url: url.to_string(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}
