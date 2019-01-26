use crate::music_service::MusicClient;
use std::collections::HashMap;

pub fn followed_artists(music_client: &MusicClient) -> HashMap<String, String> {
    music_client.user_followed_artists()
}
