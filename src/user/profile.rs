use rspotify::spotify::client::Spotify;
use std::collections::HashMap;

pub fn followed_artists(spotify: &Spotify) -> HashMap<String, String> {
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
