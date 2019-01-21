mod authentication;

use music_service::MusicClient;
use user::artist::Album;
use spotify::authentication::get_spotify_client;
use rspotify::spotify::senum::AlbumType;

pub struct SpotifyClient {}

impl MusicClient for SpotifyClient {

    fn artist_albums(&self, id: &str) -> Vec<Album> {
        let client = get_spotify_client();

        let response =
            client.artist_albums(id, Some(AlbumType::Album), None, Some(50), None);
        let albums = response.ok().unwrap().items; // TODO, check if response is different than ok

        albums
            .into_iter()
            .map(|album| {
                let url = &album.external_urls["spotify"];

                Album {
                    name: album.name,
                    id: album.id,
                    url: url.to_string()
                }
            })
            .collect::<Vec<Album>>()
    }
}