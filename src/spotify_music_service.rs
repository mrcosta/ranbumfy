mod authentication;

use crate::music_service::MusicClient;
use crate::spotify_music_service::authentication::get_spotify_client;
use crate::user::artist::Album;
use rspotify::spotify::senum::AlbumType;
use std::collections::HashMap;

pub struct SpotifyClient {}

impl MusicClient for SpotifyClient {
    fn artist_albums(&self, id: &str) -> Vec<Album> {
        let client = get_spotify_client();

        let response = client.artist_albums(id, Some(AlbumType::Album), None, Some(50), None);
        let albums = response.ok().unwrap().items; // TODO, check if response is different than ok

        albums
            .into_iter()
            .map(|album| {
                let url = &album.external_urls["spotify"];

                Album {
                    name: album.name,
                    id: album.id,
                    url: url.to_string(),
                }
            })
            .collect::<Vec<Album>>()
    }

    fn user_followed_artists(&self) -> HashMap<String, String> {
        let client = get_spotify_client();
        let mut next_request = None;
        let mut artists_and_ids = HashMap::new();

        loop {
            let response = client.current_user_followed_artists(50, next_request);
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
}
