use rand::seq::SliceRandom;
use rand::thread_rng;
use music_service::MusicClient;

pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub url: String,
}

impl Artist {
    pub fn draw_an_album(&self, music_client: &MusicClient) -> Album {
        let albums = music_client.artist_albums(&self.id);

        // TODO: how can I just randomized the album and return without creating a new album
        let randomized_album = albums.choose(&mut thread_rng()).unwrap();

        Album {
            name: randomized_album.name.to_string(),
            id: randomized_album.id.to_string(),
            url: randomized_album.url.to_string(),
        }
    }
}
