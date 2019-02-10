use rand::thread_rng;
use rand::Rng;

pub struct Artist {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub url: String,
}

impl Artist {
    pub fn draw_an_album(&self, mut albums: Vec<Album>) -> Album {
        let randomized_album_index = thread_rng().gen_range(0, albums.len());

        albums.swap_remove(randomized_album_index)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_drawn_an_album_when_artist_has_only_one_album() {
        let album = Album {
            name: "black holes and revelations".to_string(),
            id: "black_holes_123".to_string(),
            url: "https://some_url".to_string(),
        };

        let artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };

        assert_eq!(
            artist.draw_an_album(vec![album]).name,
            "black holes and revelations"
        );
    }
}
