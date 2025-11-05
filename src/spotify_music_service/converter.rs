use crate::user::artist::Album;
use crate::user::artist::Artist;
use rspotify::model::album::SimplifiedAlbum;
use rspotify::model::artist::FullArtist;
use rspotify::prelude::Id;

pub fn to_albums(spotify_albums: Vec<SimplifiedAlbum>) -> Vec<Album> {
    spotify_albums
        .into_iter()
        .map(|spotify_album| {
            let url = &spotify_album.external_urls["spotify"];

            Album {
                name: spotify_album.name,
                id: spotify_album.id.unwrap().id().to_string(),
                url: url.to_string(),
            }
        })
        .collect::<Vec<Album>>()
}

pub fn to_artists(spotify_artists: Vec<FullArtist>) -> Vec<Artist> {
    spotify_artists
        .into_iter()
        .map(|spotify_artist| Artist {
            name: spotify_artist.name,
            id: spotify_artist.id.id().to_string(),
        })
        .collect::<Vec<Artist>>()
}

#[cfg(test)]
mod test {

    use super::*;
    use rspotify::model::Type;
    use std::collections::HashMap;

    #[test]
    fn convert_from_spotify_albums_to_domain_albums() {
        let spotify_album = SimplifiedAlbum {
            artists: vec![],
            album_type: "".to_string(),
            available_markets: vec![],
            href: "".to_string(),
            images: vec![],
            _type: Type::Album,
            uri: "".to_string(),
            id: "muse_123_id".to_string(),
            name: "muse_album".to_string(),
            external_urls: [("spotify".to_string(), "http://url_to_album".to_string())]
                .iter()
                .cloned()
                .collect(),
        };

        let expected_album = Album {
            name: "muse_album".to_string(),
            id: "muse_123_id".to_string(),
            url: "http://url_to_album".to_string(),
        };

        assert_eq!(to_albums(vec![spotify_album])[0].name, expected_album.name);
    }

    #[test]
    fn convert_from_spotify_artists_to_domain_artists() {
        let spotify_artist = FullArtist {
            external_urls: HashMap::new(),
            followers: HashMap::new(),
            genres: vec![],
            href: "".to_string(),
            id: "muse_123_id".to_string(),
            images: vec![],
            name: "muse".to_string(),
            popularity: 0,
            _type: Type::Artist,
            uri: "".to_string(),
        };

        let expected_artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };

        assert_eq!(
            to_artists(vec![spotify_artist])[0].name,
            expected_artist.name
        );
    }
}
