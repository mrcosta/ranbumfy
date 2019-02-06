pub mod artist;
mod profile;

use crate::music_service::MusicClient;
use crate::user::artist::Artist;
use crate::user::profile::followed_artists;
use rand::thread_rng;
use rand::Rng;

pub fn draw_an_album_to_list(music_client: &MusicClient) {
    let artists = followed_artists(music_client);

    for _ in 0..7 {
        let randomized_artist = randomize_artist(&artists);
        let randomized_album = randomized_artist.draw_an_album(music_client);
        println!(
            "Listen to {} from {}: {}",
            randomized_album.name, randomized_artist.name, randomized_album.url
        );
    }
}

fn randomize_artist(artists: &Vec<Artist>) -> Artist {
    let randomized_artist_index = thread_rng().gen_range(0, artists.len());
    let artist = artists.get(randomized_artist_index).unwrap();

    Artist {
        id: artist.id.to_owned(),
        name: artist.name.to_owned(),
    }
}
