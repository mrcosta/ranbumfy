pub mod authentication;

use crate::music_service::MusicClient;
use crate::user::artist::Album;
use crate::user::artist::Artist;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::senum::AlbumType;

pub struct SpotifyClient {
    pub client: Spotify,
}

impl MusicClient for SpotifyClient {
    fn artist_albums(&self, id: &str) -> Vec<Album> {
        let response = self.client.artist_albums(id, Some(AlbumType::Album), None, Some(50), None);
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

    fn user_followed_artists(&self) -> Vec<Artist> {
        let mut next_request = None;
        let mut artists_and_ids = Vec::new();

        loop {
            let response = self.client.current_user_followed_artists(50, next_request);
            let artists = response.ok().unwrap().artists;

            // TODO: extract to private function
            artists
                .items
                .into_iter()
                .for_each(|artist| {
                    artists_and_ids.push(
                        Artist {
                            name: artist.name,
                            id: artist.id,
                        }
                    );
                });

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
