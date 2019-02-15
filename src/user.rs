pub mod artist;

use crate::music_service::MusicClient;
use crate::user::artist::Album;
use crate::user::artist::Artist;
use rand::thread_rng;
use rand::Rng;

pub struct UserService<'a> {
    pub music_client: &'a MusicClient,
}

impl<'a> UserService<'a> {
    pub fn draw_albums_to_list(&self) -> Vec<Album> {
        let artists = self.music_client.user_followed_artists();
        let mut drawn_albums = Vec::new();

        for _ in 0..7 {
            let randomized_artist = randomize_artist(&artists);

            let albums = self.music_client.artist_albums(&randomized_artist.id);
            let randomized_album = randomized_artist.draw_an_album(albums);
            println!(
                "Listen to {} from {}: {}",
                randomized_album.name, randomized_artist.name, randomized_album.url
            );

            drawn_albums.push(randomized_album);
        }

        drawn_albums
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

#[cfg(test)]
mod test {

    use super::*;
    use mockers::Scenario;

    #[test]
    fn test_get_followed_artist_and_drawn_7_albums() {
        let scenario = Scenario::new();
        let cond = scenario.create_mock_for::<MusicClient>();

        let artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };
        scenario.expect(cond.user_followed_artists_call().and_return(vec![artist]));

        let album = Album {
            name: "black holes and revelations".to_string(),
            id: "black_holes_123".to_string(),
            url: "https://some_url".to_string(),
        };
        scenario.expect(
            cond.artist_albums_call("muse_123_id")
                .and_return_clone(vec![album])
                .times(7),
        );

        let user_service = UserService {
            music_client: &cond,
        };

        let drawn_albums = user_service.draw_albums_to_list();

        assert_eq!(drawn_albums.len(), 7);
    }
}
