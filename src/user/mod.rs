mod artist;
mod authentication;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::spotify::client::Spotify;
use std::collections::HashMap;
use user::artist::{Artist};
use user::authentication::get_spotify;

// TODO: create artist and user traits and export the functions to there
// create unit tests that mocks the calls and integration tests from outside the library that put everything
// together

pub fn get_followed_artists() {
    let spotify = get_spotify();
    let artists_and_ids = get_all_followed_artists(&spotify);
    let artist = randomize_artist(&artists_and_ids);
    let randomized_album = artist.randomize_album(&spotify);

    println!(
        "You are going to listen to {} from {}",
        randomized_album.name, artist.name
    );
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

fn randomize_artist(artists_and_ids: &HashMap<String, String>) -> Artist {
    let artists_names = artists_and_ids.keys().cloned().collect::<Vec<String>>();
    let randomized_artist_name = artists_names.choose(&mut thread_rng()).unwrap().to_owned();
    let randomized_artist_id = &artists_and_ids[&randomized_artist_name];

    Artist {
        name: randomized_artist_name,
        id: randomized_artist_id.to_string()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}
