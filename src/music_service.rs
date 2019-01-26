#[cfg(test)]
use mockers_derive::mocked;

use crate::user::artist::Album;
use std::collections::HashMap;

#[mocked]
pub trait MusicClient {
    fn artist_albums(&self, id: &str) -> Vec<Album>;
    fn user_followed_artists(&self) -> HashMap<String, String>;
}
