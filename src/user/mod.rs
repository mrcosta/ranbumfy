pub mod artist;
mod authentication;
mod profile;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use user::artist::Artist;
use user::authentication::get_spotify;
use user::profile::followed_artists;
use music_service::MusicClient;

// TODO: create artist and user traits and export the functions to there
// create unit tests that mocks the calls and integration tests from outside the library that put everything
// together

pub fn draw_an_album_to_list(music_client: &MusicClient) {
    let spotify = get_spotify();
    let artists_and_ids = followed_artists(&spotify);
    let artist = randomize_artist(&artists_and_ids);
    let randomized_album = artist.draw_an_album(music_client);

    println!(
        "You are going to listen to {} from {}",
        randomized_album.name, artist.name
    );
    println!("Here's the url: {}", randomized_album.url);
}

fn randomize_artist(artists_and_ids: &HashMap<String, String>) -> Artist {
    let artists_names = artists_and_ids.keys().cloned().collect::<Vec<String>>();
    let randomized_artist_name = artists_names.choose(&mut thread_rng()).unwrap().to_owned();
    let randomized_artist_id = &artists_and_ids[&randomized_artist_name];

    Artist {
        name: randomized_artist_name,
        id: randomized_artist_id.to_string(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}

// questions:
// hashmap of string or &str given the context of doing requests that returns String??
// when mod stand alone or create a trait??
// how piramid tests works? would I unit test from each single module and integration test from the mod.rs??
// https://developer.spotify.com/documentation/web-api/quick-start/
