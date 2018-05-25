use std::sync::{Arc, Mutex};
use enigo::Enigo;

use notemappings::NoteMappings;

/// The object that gets passed to the MIDI callback, containing all our state
#[derive(Clone)]
pub struct AppState {
    keygen: Arc<Mutex<Enigo>>,
    mappings: Arc<Mutex<NoteMappings>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            keygen: Arc::new(Mutex::new(Enigo::new())),
            mappings: Arc::new(Mutex::new(NoteMappings::new())),
        }
    }

    pub fn keygen(&self) -> &Arc<Mutex<Enigo>> {
        &self.keygen
    }

    pub fn mappings(&self) -> &Arc<Mutex<NoteMappings>> {
        &self.mappings
    }
}