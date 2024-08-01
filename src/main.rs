use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use base64::encode;
use tokio;

#[derive(Serialize, Deserialize)]
struct Track {
    artist: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
struct SpotifyToken {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct SpotifySearchResponse {
    tracks: SpotifyTracks,
}

#[derive(Deserialize)]
struct SpotifyTracks {
    items: Vec<SpotifyTrack>,
}

#[derive(Deserialize)]
struct SpotifyTrack {
    uri: String,
}

async fn get_access_token() -> Result<SpotifyToken, Box<dyn std::error::Error>> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")?;
    let auth_code = env::var("SPOTIFY_AUTH_CODE")?;

    let auth = encode(format!("{}:{}", client_id, client_secret).as_bytes());

    let client = reqwest::Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &auth_code),
        ("redirect_uri", &redirect_uri),
    ];

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header(AUTHORIZATION, format!("Basic {}", auth))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .json::<SpotifyToken>()
        .await?;

    Ok(res)
}
async fn search_track(
    access_token: &str,
    artist: &str,
    title: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let query = format!("artist:{} track:{}", artist, title);

    let res = client
        .get("https://api.spotify.com/v1/search")
        .query(&[("q", &query), ("type", &"track".to_string()), ("limit", &"1".to_string())])
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<SpotifySearchResponse>()
        .await?;

    Ok(res.tracks.items.first().map(|track| track.uri.clone()))
}

async fn add_track_to_playlist(
    access_token: &str,
    playlist_id: &str,
    track_uri: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let uris = vec![track_uri];
    let body = serde_json::json!({ "uris": uris });

    client
        .post(&format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let tracks_json = fs::read_to_string("tracks.json")?;
    let tracks: Vec<Track> = serde_json::from_str(&tracks_json)?;

    let spotify_token = get_access_token().await?;
    let access_token = spotify_token.access_token;
    let mut not_found_tracks = vec![];

    for track in tracks {
        if let Some(track_uri) = search_track(&access_token, &track.artist, &track.title).await? {
            let playlist_id = env::var("SPOTIFY_PLAYLIST_ID")?;
            add_track_to_playlist(&access_token, &playlist_id, &track_uri).await?;
            println!("Track added successfully: {} - {}", track.artist, track.title);
        } else {
            println!("Track not found: {} - {}", track.artist, track.title);
            not_found_tracks.push(track);
        }
    }

    if !not_found_tracks.is_empty() {
        let not_found_json = serde_json::to_string_pretty(&not_found_tracks)?;
        fs::write("not_found_tracks.json", not_found_json)?;
        println!("Not found tracks written to not_found_tracks.json");
    }

    Ok(())
}
