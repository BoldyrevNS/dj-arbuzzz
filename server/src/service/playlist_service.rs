use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::{
    dto::response::websocket::{PlaylistData, PlaylistItemData, WebSocketMessage},
    error::app_error::AppResult,
    infrastucture::cache::{client::Cache, keys::AppCacheKey},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaylistItem {
    pub id: i32,
    pub song_id: i32,
    pub owner_id: i32,
    pub artist: String,
    pub title: String,
    pub duration_sec: i32,
    pub download_url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,
}

const WS_EVENT_CAPACITY: usize = 100;

pub struct PlaylistService {
    cache: Arc<Cache>,
    ws_event_sender: broadcast::Sender<WebSocketMessage>,
}

impl PlaylistService {
    pub fn new(cache: Arc<Cache>) -> Self {
        let (ws_event_sender, _) = broadcast::channel(WS_EVENT_CAPACITY);
        PlaylistService { 
            cache,
            ws_event_sender,
        }
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.ws_event_sender.subscribe()
    }

    async fn notify_playlist_changed(&self) -> AppResult<()> {
        let playlist = self.get_playlist().await?;
        let items: Vec<PlaylistItemData> = playlist.items.iter().map(|item| PlaylistItemData {
            artist: item.artist.clone(),
            title: item.title.clone(),
            duration_sec: item.duration_sec,
        }).collect();
        let msg = WebSocketMessage::Playlist(PlaylistData { items });
        let _ = self.ws_event_sender.send(msg);
        Ok(())
    }

    pub async fn get_playlist_ws(&self) -> AppResult<PlaylistData> {
        let playlist = self.get_playlist().await?;
        let items: Vec<PlaylistItemData> = playlist.items.iter().map(|item| PlaylistItemData {
            artist: item.artist.clone(),
            title: item.title.clone(),
            duration_sec: item.duration_sec,
        }).collect();
        Ok(PlaylistData { items })
    }

    pub async fn add_new_track(&self, item: PlaylistItem) -> AppResult<()> {
        let key = AppCacheKey::PLAYLIST().build_key();
        let mut con = self.cache.get_async_conn().await?;
        let playlist = match con.get::<String, String>(key.clone()).await {
            Ok(playlist_str) => serde_json::from_str::<Playlist>(&playlist_str)?,
            Err(_) => Playlist { items: vec![] },
        };
        let mut new_playlist = playlist;
        new_playlist.items.push(item);
        let playlist_str = serde_json::to_string(&new_playlist)?;
        let _: () = con.set(key, playlist_str).await?;
        self.notify_playlist_changed().await?;
        Ok(())
    }

    pub async fn pop_track(&self) -> AppResult<PlaylistItem> {
        let key = AppCacheKey::PLAYLIST().build_key();
        let mut con = self.cache.get_async_conn().await?;
        let playlist = match con.get::<String, String>(key.clone()).await {
            Ok(playlist_str) => serde_json::from_str::<Playlist>(&playlist_str)?,
            Err(_) => Playlist { items: vec![] },
        };
        let mut new_playlist = playlist;
        if new_playlist.items.is_empty() {
            return Err(crate::error::app_error::AppError::NotFound(
                "Playlist is empty".to_string(),
                None,
            ));
        }
        let item = new_playlist.items.remove(0);
        let playlist_str = serde_json::to_string(&new_playlist)?;
        let _: () = con.set(key, playlist_str).await?;
        let _ = self.notify_playlist_changed().await;
        Ok(item)
    }

    pub async fn get_playlist(&self) -> AppResult<Playlist> {
        let key = AppCacheKey::PLAYLIST().build_key();
        let mut con = self.cache.get_async_conn().await?;
        let playlist = match con.get::<String, String>(key).await {
            Ok(playlist_str) => serde_json::from_str::<Playlist>(&playlist_str)?,
            Err(_) => Playlist { items: vec![] },
        };
        Ok(playlist)
    }
}
