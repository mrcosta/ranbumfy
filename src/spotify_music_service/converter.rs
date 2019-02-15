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

#[cfg(test)]
mod test {

    use super::*;
    use rspotify::spotify::senum::Type;

    #[test]
    fn convert_from_spotify_simplified_to_domain_albums() {
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
                external_urls: [("spotify".to_string(), "http://url_to_album".to_string())].iter().cloned().collect(),
        };

        let expected_album = Album {
            name: "muse_album".to_string(),
            id: "muse_123_id".to_string(),
            url: "http://url_to_album".to_string()
        };

        assert_eq!(to_albums(vec![spotify_album])[0].name, expected_album.name);
    }
}