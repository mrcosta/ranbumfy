pub mod artist;
mod profile;

use crate::music_service::MusicClient;
use crate::user::artist::Artist;
use crate::user::profile::followed_artists;
use rand::thread_rng;
use rand::Rng;

pub fn draw_an_album_to_list(music_client: &MusicClient) {
    let artists = followed_artists(music_client);
    let randomized_artist = randomize_artist(artists);
    let randomized_album = randomized_artist.draw_an_album(music_client);

    println!(
        "You are going to listen to {} from {}",
        randomized_album.name, randomized_artist.name
    );
    println!("Here's the url: {}", randomized_album.url);
}

fn randomize_artist(mut artists: Vec<Artist>) -> Artist {
    let randomized_artist_index = thread_rng().gen_range(0, artists.len());
    artists.swap_remove(randomized_artist_index)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_zero() {
        assert_eq!("zero", "zero");
    }
}

// TODO: create artist and user traits and export the functions to there
