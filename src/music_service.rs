#[cfg(test)]
use mockers_derive::mocked;

use crate::user::artist::Album;

#[mocked]
pub trait MusicClient {
    fn artist_albums(&self, id: &str) -> Vec<Album>;
}
