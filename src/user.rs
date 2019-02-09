pub mod artist;

use crate::music_service::MusicClient;
use crate::user::artist::Artist;
use rand::thread_rng;
use rand::Rng;

pub struct UserService<'a> {
    pub music_client: &'a MusicClient,
}

impl<'a> UserService<'a> {

    pub fn draw_an_album_to_list(&self) {
        let artists = self.music_client.user_followed_artists();

        for _ in 0..7 {
            let randomized_artist = randomize_artist(&artists);

            let albums = self.music_client.artist_albums(&randomized_artist.id);
            let randomized_album = randomized_artist.draw_an_album(albums);
            println!(
                "Listen to {} from {}: {}",
                randomized_album.name, randomized_artist.name, randomized_album.url
            );
        }
    }
}

fn randomize_artist(artists: &[Artist]) -> Artist {
    let randomized_artist_index = thread_rng().gen_range(0, artists.len());
    let artist = &artists[randomized_artist_index];

    Artist {
        id: artist.id.to_owned(),
        name: artist.name.to_owned(),
    }
}
