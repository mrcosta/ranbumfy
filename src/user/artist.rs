use crate::music_service::MusicClient;
use rand::thread_rng;
use rand::Rng;

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
        let mut albums = music_client.artist_albums(&self.id);

        let randomized_album_index = thread_rng().gen_range(0, albums.len());
        albums.swap_remove(randomized_album_index)
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
            url: "https://some_url".to_string(),
        };
        scenario.expect(
            cond.artist_albums_call("muse_123_id")
                .and_return(vec![album]),
        );

        let artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };

        assert_eq!(
            artist.draw_an_album(&mut cond).name,
            "black holes and revelations"
        );
    }
}
