use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Track {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_secs: Option<u64>,
}

#[derive(Clone, serde::Serialize)]
pub struct PlaybackStatus {
    pub position_secs: f64,
    pub duration_secs: f64,
    pub is_playing: bool,
}