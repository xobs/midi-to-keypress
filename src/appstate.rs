use std::sync::{Arc, Mutex};
use std::collections::HashMap;

extern crate enigo;
use enigo::{Enigo, KeyboardControllable};

use notemappings::{NoteMappings, KbdKey};

pub struct KeyGen {
    enigo: Enigo,
    key_state: HashMap<KbdKey, bool>,
}

impl KeyGen {
    pub fn new() -> KeyGen {
        KeyGen {
            enigo: Enigo::new(),
            key_state: HashMap::new(),
        }
    }

    /// Press a given key.
    /// Returns `true` if an event was sent.
    pub fn key_down(&mut self, key: &KbdKey) -> bool {
        if let Some(val) = self.key_state.get(key) {
            if *val == true {
                return false;
            }
        }
        self.key_state.insert(key.clone(), true);
        self.enigo.key_down(KbdKey::to_enigo_key(key));
        true
    }

    /// Release a given key.
    /// Returns `true` if an event was sent.
    pub fn key_up(&mut self, key: &KbdKey) -> bool {
        if let Some(val) = self.key_state.get(key) {
            if *val == false {
                return false;
            }
        }
        self.enigo.key_up(KbdKey::to_enigo_key(key));
        self.key_state.insert(key.clone(), false);
        true
    }

    /// Returns the number of keys that were reset
    pub fn key_reset(&mut self) -> u32 {
        let mut changes = 0;
        for (key, pressed) in &self.key_state {
            if *pressed {
                self.enigo.key_up(KbdKey::to_enigo_key(key));
                changes = changes + 1;
            }
        }

        self.key_state.clear();
        changes
    }
}

/// The object that gets passed to the MIDI callback, containing all our state
#[derive(Clone)]
pub struct AppState {
    keygen: Arc<Mutex<KeyGen>>,
    mappings: Arc<Mutex<NoteMappings>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            keygen: Arc::new(Mutex::new(KeyGen::new())),
            mappings: Arc::new(Mutex::new(NoteMappings::new())),
        }
    }

    pub fn keygen(&self) -> &Arc<Mutex<KeyGen>> {
        &self.keygen
    }

    pub fn mappings(&self) -> &Arc<Mutex<NoteMappings>> {
        &self.mappings
    }
}
