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

    // Try to read token from cache
    let token_result = spotify.read_token_cache(true).await;

    match token_result {
        Ok(Some(token)) => {
            println!("Using cached token");
            *spotify.get_token().lock().await.unwrap() = Some(token);
        }
        _ => {
            // Request new token
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
                    eprintln!("2. Copy the FULL redirect URL (http://localhost/?code=...)");
                    eprintln!("3. Paste it when prompted");
                    eprintln!("\nAlso verify 'http://localhost' is in your Spotify app's Redirect URIs");
                    std::process::exit(1);
                }
            }
        }
    }

    spotify
}
