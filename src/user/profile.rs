use crate::music_service::MusicClient;
use crate::user::artist::Artist;

pub fn followed_artists(music_client: &MusicClient) -> Vec<Artist> {
    music_client.user_followed_artists()
}
