pub mod artist;

use crate::music_service::MusicClient;
use crate::user::artist::Album;
use crate::user::artist::Artist;
use rand::thread_rng;
use rand::Rng;
use std::path::PathBuf;
use std::fs;

pub struct UserService {
    pub music_client: Box<dyn MusicClient>,
}

impl UserService {
    async fn get_followed_artists_cached(&self) -> Vec<Artist> {
        let cache_path = get_cache_path();

        // Try to load from cache first
        if let Ok(cached_artists) = load_artists_from_cache(&cache_path) {
            println!("Using cached artists (delete {} to refresh)", cache_path.display());
            return cached_artists;
        }

        // Cache miss or invalid, fetch from API
        println!("Fetching followed artists from Spotify...");
        let artists = self.music_client.user_followed_artists().await;

        // Save to cache
        if let Err(e) = save_artists_to_cache(&cache_path, &artists) {
            eprintln!("Warning: Failed to cache artists: {}", e);
        } else {
            println!("Cached {} artists to {}", artists.len(), cache_path.display());
        }

        artists
    }

    pub async fn draw_albums_to_list(&self) -> Vec<Album> {
        let artists = self.get_followed_artists_cached().await;

        println!("Fetching 15 random albums...\n");

        // Generate 15 random artists upfront
        let mut selected_artists = Vec::new();
        for _ in 0..15 {
            selected_artists.push(randomize_artist(&artists));
        }

        // Fetch all albums in parallel using futures
        let album_futures: Vec<_> = selected_artists
            .iter()
            .map(|artist| {
                let artist_id = artist.id.clone();
                async move {
                    self.music_client.artist_albums(&artist_id).await
                }
            })
            .collect();

        // Wait for all requests to complete
        let album_results = futures::future::join_all(album_futures).await;

        // Process results and print
        let mut drawn_albums = Vec::new();
        let mut count = 1;

        for (artist, albums) in selected_artists.iter().zip(album_results.iter()) {
            // Print artist even if they have no albums
            if albums.is_empty() {
                println!("Listen to {} (no albums available on Spotify)", artist.name);
                continue;
            }

            let randomized_album = artist.draw_an_album(albums.clone());
            println!(
                "{}. Listen to {} from {}: {}",
                count,
                randomized_album.name,
                artist.name,
                randomized_album.url
            );

            drawn_albums.push(randomized_album);
            count += 1;
        }

        println!("\nDone! Found {} albums to listen to.", drawn_albums.len());
        drawn_albums
    }
}

fn randomize_artist(artists: &[Artist]) -> Artist {
    let randomized_artist_index = thread_rng().gen_range(0..artists.len());
    let artist = &artists[randomized_artist_index];

    Artist {
        id: artist.id.to_owned(),
        name: artist.name.to_owned(),
    }
}

fn get_cache_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push(".ranbumfy_artists_cache.json");
    path
}

fn load_artists_from_cache(path: &PathBuf) -> Result<Vec<Artist>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let artists: Vec<Artist> = serde_json::from_str(&contents)?;
    Ok(artists)
}

fn save_artists_to_cache(path: &PathBuf, artists: &[Artist]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(artists)?;
    fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use mockers::Scenario;

    #[test]
    fn test_get_followed_artist_and_drawn_15_albums() {
        let scenario = Scenario::new();
        let cond = scenario.create_mock_for::<MusicClient>();

        let artist = Artist {
            name: "muse".to_string(),
            id: "muse_123_id".to_string(),
        };
        scenario.expect(cond.user_followed_artists_call().and_return(vec![artist]));

        let album = Album {
            name: "black holes and revelations".to_string(),
            id: "black_holes_123".to_string(),
            url: "https://some_url".to_string(),
        };
        scenario.expect(
            cond.artist_albums_call("muse_123_id")
                .and_return_clone(vec![album])
                .times(15),
        );

        let user_service = UserService {
            music_client: Box::new(cond),
        };

        let drawn_albums = user_service.draw_albums_to_list();

        assert_eq!(drawn_albums.len(), 15);
    }
}
