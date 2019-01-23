use crate::music_service::MusicClient;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

#[cfg(test)]
mod test {

    use super::*;
    use mockers::Scenario;

    #[test]
    fn test_drawn_an_album_when_artist_has_only_one_album() {
        let scenario = Scenario::new();
        let mut cond = scenario.create_mock_for::<MusicClient>();

        let album = Album {
            name: "black holes and revelations".to_string(),
            id: "black_holes_123".to_string(),
            url: "https://some_url".to_string()
        };
        scenario.expect(cond.artist_albums_call("muse_123_id").and_return(vec![album]));

        let artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };

        assert_eq!(artist.draw_an_album(&mut cond).name, "black holes and revelations");
    }
}
