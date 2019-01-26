pub mod authentication;

use crate::music_service::MusicClient;
use crate::user::artist::Album;
use crate::user::artist::Artist;
use log::{info};
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::FullArtist;
use rspotify::spotify::senum::AlbumType;

pub struct SpotifyClient {
    pub client: Spotify,
}

impl MusicClient for SpotifyClient {
    fn artist_albums(&self, id: &str) -> Vec<Album> {
        let response = self.client.artist_albums(id, Some(AlbumType::Album), None, Some(50), None);
        let albums = response.ok().expect("didn't return expected response").items;

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
        let mut followed_artists = Vec::new();

        loop {
            let response = self.client.current_user_followed_artists(50, next_request);
            let artists = response.ok().unwrap().artists;

            fill_followed_artists(artists.items, &mut followed_artists);

            next_request = artists.cursors.after;
            if next_request.is_none() {
                break;
            } else {
                info!("Doing next request: {:?}", &next_request);
            }
        }

        followed_artists
    }
}

fn fill_followed_artists(artists: Vec<FullArtist>, followed_artists: &mut Vec<Artist>) {
    artists
        .into_iter()
        .for_each(|artist| {
            followed_artists.push(
                Artist {
                    name: artist.name,
                    id: artist.id,
                }
            );
        });
}
