pub mod authentication;
mod converter;

use crate::music_service::MusicClient;
use crate::spotify_music_service::converter::{to_albums, to_artists};
use crate::user::artist::Album;
use crate::user::artist::Artist;
use rspotify::AuthCodeSpotify;
use rspotify::model::artist::FullArtist;
use rspotify::model::AlbumType;
use rspotify::model::ArtistId;
use rspotify::clients::{BaseClient, OAuthClient};
use async_trait::async_trait;

pub struct SpotifyClient {
    pub client: AuthCodeSpotify,
}

#[async_trait]
impl MusicClient for SpotifyClient {
    async fn artist_albums(&self, id: &str) -> Vec<Album> {
        let artist_id = ArtistId::from_id(id).unwrap();
        // Use manual method to fetch a single page directly (faster than streaming)
        let result = self.client.artist_albums_manual(
            artist_id,
            Some(AlbumType::Album),
            None,
            Some(10), // limit
            None      // offset
        ).await;

        let spotify_albums = match result {
            Ok(page) => page.items,
            Err(_) => Vec::new(),
        };

        to_albums(spotify_albums)
    }

    async fn user_followed_artists(&self) -> Vec<Artist> {
        let mut next_request: Option<String> = None;
        let mut followed_artists = Vec::new();

        loop {
            let response = self.client.current_user_followed_artists(next_request.as_deref(), None).await;
            let page = response.ok().unwrap();

            fill_followed_artists(page.items.clone(), &mut followed_artists);

            next_request = page.cursors.and_then(|c| c.after);
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
