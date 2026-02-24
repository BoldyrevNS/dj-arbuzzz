use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    #[serde(rename = "current_track")]
    CurrentTrack(CurrentTrackData),
    #[serde(rename = "playlist")]
    Playlist(PlaylistData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentTrackData {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistData {
    pub items: Vec<PlaylistItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItemData {
    pub artist: String,
    pub title: String,
    pub duration_sec: i32,
}
