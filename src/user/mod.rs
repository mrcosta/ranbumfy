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
        .map(|v| v.clone())
        .collect::<Vec<String>>();

    let randomized_artist_name = artists_names.choose(& mut thread_rng()).unwrap();
    let artist_id = artists_and_ids.get(randomized_artist_name).unwrap();
    println!("You are going to listen to {}", randomized_artist_name);
//    let albums = get_artist_albums(&spotify, artists_and_ids);
//    get_artist_albums_ids(&spotify, artists_and_ids);
//    let randomized_artist = get_random_artist();
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

fn get_artist_albums(
    spotify: &Spotify,
    artists_and_ids: HashMap<String, String>,
) -> Vec<Artist> {
    let mut artists = Vec::new();

    println!("Number of artists: {}", artists_and_ids.iter().len());
    for (name, id) in artists_and_ids {
        let response = spotify.artist_albums(&id, Some(AlbumType::Album), None, Some(50), None);
        let albums_response = response.ok().unwrap().items;

        let albums = albums_response
            .into_iter()
            .map(|album| Album {
                             name: album.name,
                             id: album.id
            })
            .collect::<Vec<Album>>();

        let artist = Artist { name, id, albums };
        println!("Got the follow albums from {}: {:?}", artist.name, artist.albums);
        artists.push(artist);
    }

    artists
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}
