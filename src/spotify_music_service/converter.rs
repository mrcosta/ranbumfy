use crate::user::artist::Album;
use rspotify::spotify::model::album::SimplifiedAlbum;

pub fn to_albums(spotify_albums: Vec<SimplifiedAlbum>) -> Vec<Album> {
    spotify_albums
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