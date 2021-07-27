use crate::midi::MidiNote;
use enigo::Key;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Proxy for Enigo::Key, since that variant isn't cloneable
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum KbdKey {
    /// return key
    Return,
    /// tab key (tabulator)
    Tab,
    /// space key
    Space,
    /// backspace key
    Backspace,
    /// escape key (esc)
    Escape,
    /// super key on linux command key on macOS, windows key on Windows
    Meta,
    /// shift key
    Shift,
    /// caps lock key
    CapsLock,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    /// control key
    Control,
    /// home key
    Home,
    /// page up key
    PageUp,
    /// page down key
    PageDown,
    /// left arrow key
    LeftArrow,
    /// right arrow key
    RightArrow,
    /// down arrow key
    DownArrow,
    /// up arrow key
    UpArrow,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// keyboard layout dependent key
    Layout(char),
    /// raw keycode eg 0x38
    Raw(u16),
}

impl KbdKey {
    pub fn to_enigo_key(obj: &KbdKey) -> Key {
        match *obj {
            KbdKey::Return => Key::Return,
            KbdKey::Tab => Key::Tab,
            KbdKey::Space => Key::Space,
            KbdKey::Backspace => Key::Backspace,
            KbdKey::Escape => Key::Escape,
            KbdKey::Meta => Key::Meta,
            KbdKey::Shift => Key::Shift,
            KbdKey::CapsLock => Key::CapsLock,
            KbdKey::Alt => Key::Alt,
            KbdKey::Option => Key::Option,
            KbdKey::Control => Key::Control,
            KbdKey::Home => Key::Home,
            KbdKey::PageUp => Key::PageUp,
            KbdKey::PageDown => Key::PageDown,
            KbdKey::LeftArrow => Key::LeftArrow,
            KbdKey::RightArrow => Key::RightArrow,
            KbdKey::DownArrow => Key::DownArrow,
            KbdKey::UpArrow => Key::UpArrow,
            KbdKey::F1 => Key::F1,
            KbdKey::F2 => Key::F2,
            KbdKey::F3 => Key::F3,
            KbdKey::F4 => Key::F4,
            KbdKey::F5 => Key::F5,
            KbdKey::F6 => Key::F6,
            KbdKey::F7 => Key::F7,
            KbdKey::F8 => Key::F8,
            KbdKey::F9 => Key::F9,
            KbdKey::F10 => Key::F10,
            KbdKey::F11 => Key::F11,
            KbdKey::F12 => Key::F12,
            KbdKey::Layout(c) => Key::Layout(c),
            KbdKey::Raw(c) => Key::Raw(c),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    /// Insert a Delay for a specified number of ms
    Delay(u64),

    /// Press a key
    KeyDown(KbdKey),

    /// Release a key
    KeyUp(KbdKey),

    /// Keep a key held down during this script.
    /// Note that the key may be continued to be held down until a script
    /// with no NoteMod is encountered.
    NoteMod(Option<KbdKey>),
}

#[derive(Clone, Debug)]
pub struct NoteMapping {
    /// The source note that triggered this event.
    note: MidiNote,

    /// The source channel.  0 is a good default here.
    channel: u8,

    /// The name of the instrument that we're looking for.
    instrument_name: Option<String>,

    /// A sequence to call when the note is pressed.
    pub on: Vec<Event>,

    /// A sequence to call when the note is released.
    pub off: Vec<Event>,
}

impl NoteMapping {
    pub fn new(note: MidiNote, channel: u8, instrument_name: Option<String>) -> NoteMapping {
        NoteMapping {
            note,
            channel,
            instrument_name,
            on: vec![],
            off: vec![],
        }
    }

    pub fn down_event(key: char, modifier: Option<KbdKey>, _delay: Option<u64>) -> Vec<Event> {
        let mut v = vec![];

        if let Some(ref m) = modifier {
            v.push(Event::NoteMod(Some(m.clone())));
        } else {
            v.push(Event::NoteMod(None));
        }

        v.push(Event::KeyDown(KbdKey::Layout(key)));

        v
    }

    pub fn up_event(key: char, _modifier: Option<KbdKey>, _delay: Option<u64>) -> Vec<Event> {
        /*
        let mut v = vec![];
                if let Some(ref m) = modifier {
                    v.push(Event::KeyUp(m.clone()));
                }

                if let Some(d) = delay {
                    v.push(Event::Delay(d));
                }
                v.push(Event::KeyUp(KbdKey::Layout(key)));
        */
        vec![Event::KeyUp(KbdKey::Layout(key))]
    }
}

#[derive(Default)]
pub struct NoteMappings {
    mappings: Vec<NoteMapping>,
}

impl NoteMappings {
    pub fn new() -> NoteMappings {
        NoteMappings::default()
    }

    /// Find a mapping for a given note, if one exists
    pub fn find(
        &self,
        note: MidiNote,
        channel: u8,
        instrument_name: Option<String>,
    ) -> Option<NoteMapping> {
        for mapping in &self.mappings {
            if mapping.note == note
                && mapping.channel == channel
                && mapping.instrument_name == instrument_name
            {
                return Some(mapping.clone());
            }
        }
        None
    }

    pub fn import(&mut self, filename: &str) -> Result<()> {
        let f = File::open(filename)?;
        let buf_reader = BufReader::new(f);
        for line in buf_reader.lines() {
            let l = line.unwrap();
            let fields: Vec<&str> = l.split(' ').collect();
            if fields.len() != 4 {
                println!("Line is not 4 elements!");
                continue;
            }
            let note_txt = fields[0];
            let channel_txt = fields[1];
            let keydown_txt = fields[2];
            let keyup_txt = fields[3];

            let note = MidiNote::new_from_text(&note_txt).unwrap();
            let channel = channel_txt.parse::<u8>().unwrap();
            let keydown = keydown_txt.chars().next().unwrap();
            let keyup = keyup_txt.chars().next().unwrap();

            let mut mapping = NoteMapping::new(note, channel, None);
            mapping.on = NoteMapping::down_event(keydown, None, None);
            mapping.off = NoteMapping::up_event(keyup, None, None);

            println!("Got line: {}  Mapping: {:?}", l, mapping);
            self.add(mapping);
        }
        Ok(())
    }

    pub fn add(&mut self, mapping: NoteMapping) {
        //Note: We need to remove old mappings here, too!
        self.mappings.push(mapping);
    }
}
