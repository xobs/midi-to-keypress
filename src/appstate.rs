use std::sync::{Arc, Mutex};
use enigo::Enigo;

/// The object that gets passed to the MIDI callback, containing all our state
#[derive(Clone)]
pub struct AppState {
    keygen: Arc<Mutex<Enigo>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            keygen: Arc::new(Mutex::new(Enigo::new())),
        }
    }

    pub fn keygen(&self) -> &Arc<Mutex<Enigo>> {
        &self.keygen
    }
}