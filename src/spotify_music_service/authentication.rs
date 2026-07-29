use rspotify::{
    AuthCodeSpotify,
    Credentials,
    OAuth,
    scopes,
    clients::{BaseClient, OAuthClient}
};

pub async fn get_spotify_client() -> AuthCodeSpotify {
    let creds = Credentials::from_env().expect("RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET must be set");
    let oauth = OAuth::from_env(scopes!("user-follow-read")).expect("RSPOTIFY_REDIRECT_URI must be set");

    let config = rspotify::Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    // Expired tokens are read from the cache as well, so they can be refreshed
    // with their refresh token instead of asking the user to authorize again.
    if let Ok(Some(token)) = spotify.read_token_cache(true).await {
        let expired = token.is_expired();
        *spotify.get_token().lock().await.unwrap() = Some(token);

        if !expired {
            println!("Using cached token");
            return spotify;
        }

        println!("Cached token expired, refreshing it...");
        match spotify.refresh_token().await {
            Ok(_) => {
                println!("Token refreshed");
                return spotify;
            }
            Err(e) => {
                // Spotify revokes refresh tokens after long inactivity or when
                // the app credentials change: fall back to a full authorization.
                eprintln!("Could not refresh the cached token: {}", e);
                *spotify.get_token().lock().await.unwrap() = None;
            }
        }
    }

    authorize(&spotify).await;

    spotify
}

async fn authorize(spotify: &AuthCodeSpotify) {
    println!("\nOpening browser for Spotify authorization...");
    let url = spotify.get_authorize_url(false).unwrap();
    println!("If browser doesn't open, visit: {}", url);

    match spotify.prompt_for_token(&url).await {
        Ok(_) => {
            println!("Authorization successful!");
            spotify.write_token_cache().await.ok();
        }
        Err(e) => {
            eprintln!("\nAuthentication failed: {}", e);
            eprintln!("\nMake sure to:");
            eprintln!("1. Authorize the app in your browser");
            eprintln!("2. Copy the FULL redirect URL (http://127.0.0.1:8888/callback?code=...)");
            eprintln!("3. Paste it when prompted");
            eprintln!("\nAlso verify 'http://localhost' is in your Spotify app's Redirect URIs");
            std::process::exit(1);
        }
    }
}
