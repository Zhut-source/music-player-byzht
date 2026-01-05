
use rodio::Sink;
use std::sync::Mutex;

pub struct AudioPlayerState {
    pub sink: Mutex<Sink>,
}