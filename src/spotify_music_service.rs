pub mod authentication;
mod converter;

use crate::music_service::MusicClient;
use crate::spotify_music_service::converter::{to_albums, to_artists};
use crate::user::artist::Album;
use crate::user::artist::Artist;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::FullArtist;
use rspotify::spotify::senum::AlbumType;

pub struct SpotifyClient {
    pub client: Spotify,
}

impl MusicClient for SpotifyClient {
    fn artist_albums(&self, id: &str) -> Vec<Album> {
        let response = self
            .client
            .artist_albums(id, Some(AlbumType::Album), None, Some(50), None);
        let spotify_albums = response.expect("didn't return expected response").items;

        to_albums(spotify_albums)
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
            }
        }

        followed_artists
    }
}

fn fill_followed_artists(artists: Vec<FullArtist>, followed_artists: &mut Vec<Artist>) {
    to_artists(artists).into_iter().for_each(|artist| {
        followed_artists.push(artist);
    });
}
