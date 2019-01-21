use user::artist::Album;

pub trait MusicClient {
    fn artist_albums(&self, id: &str) -> Vec<Album>;
}
