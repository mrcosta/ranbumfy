#[cfg(test)] use mockers_derive::mocked;

use crate::user::artist::Album;
use crate::user::artist::Artist;
use async_trait::async_trait;

#[async_trait]
#[cfg_attr(test, mocked)]
pub trait MusicClient: Send + Sync {
    async fn artist_albums(&self, id: &str) -> Vec<Album>;
    async fn user_followed_artists(&self) -> Vec<Artist>;
}
