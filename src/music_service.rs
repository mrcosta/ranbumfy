#[cfg(test)] use mockers_derive::mocked;

use crate::user::artist::Album;
use crate::user::artist::Artist;

#[cfg_attr(test, mocked)]
pub trait MusicClient {
    fn artist_albums(&self, id: &str) -> Vec<Album>;
    fn user_followed_artists(&self) -> Vec<Artist>;
}
