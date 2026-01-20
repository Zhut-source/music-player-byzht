
use rodio::Sink;
use std::sync::{Arc, Mutex};

pub struct AudioPlayerState {
    pub sink: Arc<Mutex<Sink>>,
}