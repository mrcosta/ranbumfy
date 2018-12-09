use rand::seq::SliceRandom;
use rand::thread_rng;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::senum::AlbumType;

pub struct Artist {
    pub name: String,
    pub id: String
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub url: String,
}

impl Artist {
    pub fn randomize_album(&self, spotify: &Spotify) -> Album {
        let response = spotify.artist_albums(&self.id, Some(AlbumType::Album), None, Some(50), None);
        let albums = response.ok().unwrap().items;

        let randomized_album_from_response = albums.choose(&mut thread_rng()).unwrap();
        let name = &randomized_album_from_response.name;
        let id = &randomized_album_from_response.id;
        let url = &randomized_album_from_response.external_urls["spotify"];

        Album {
            name: name.to_string(),
            id: id.to_string(),
            url: url.to_string(),
        }
    }
}
