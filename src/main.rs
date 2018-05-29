#![windows_subsystem = "console"]

extern crate clap;
extern crate enigo;
extern crate midir;

use std::error::Error;
use std::time::Duration;
use std::thread;
use std::fmt::Write;

use clap::{App, Arg};

use midir::{Ignore, MidiInput, MidiInputConnection};

mod midi;
use midi::{MidiMessage, MidiEvent, MidiNote};

mod appstate;
use appstate::AppState;

mod notemappings;
use notemappings::{Event, KbdKey, NoteMapping, NoteMappings};

/// The amount of time to wait for a keyboard modifier to stick
const MOD_DELAY_MS: u64 = 150;

/// The amount of time to wait for a keydown event to stick
const KEY_DELAY_MS: u64 = 40;

/// The amount of time required for system events, such as Esc
const SYS_DELAY_MS: u64 = 400;

/// A small delay required when switching between octaves.
const OCTAVE_DELAY_MS: u64 = 10;

fn main() {
    let matches = App::new("Midi Perform")
        .version("0.2.9")
        .author("Sean Cross <sean@xobs.io>")
        .about("Accepts MIDI controller data and simulates keyboard presses")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List available devices"),
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .help("Connect to specified device")
                .value_name("DEVICE"),
        )
        .get_matches();

    if matches.is_present("list") {
        list_devices().expect("unable to list MIDI devices");
        return;
    }

    let device_name = match matches.value_of("device") {
        Some(s) => Some(s.to_owned()),
        None => None,
    };
    run(device_name).unwrap();
}

/// This function is called for every message that gets passed in.
fn midi_callback(_timestamp_us: u64, raw_message: &[u8], app_state: &AppState) {
    let mut keygen = app_state.keygen().lock().unwrap();
 
    if let Ok(msg) = MidiMessage::new(raw_message) {
        if let Some(note_mapping) = app_state.mappings().lock().unwrap().find(msg.note(), msg.channel(), None) {
            let sequence = match msg.event() {
                &MidiEvent::NoteOn => &note_mapping.on,
                &MidiEvent::NoteOff => &note_mapping.off,
            };

            //println!("Found note mapping: {:?} for event {:?}, running sequence {:?}", note_mapping, msg.event(), sequence);
            for event in sequence {
                match event {
                    &notemappings::Event::Delay(msecs) => thread::sleep(Duration::from_millis(msecs)),
                    &notemappings::Event::KeyDown(ref k) => {keygen.key_down(&k);},
                    &notemappings::Event::KeyUp(ref k) => {keygen.key_up(&k);},

                    // For NoteMod, which goes at the top of a note, see if we need to change
                    // the current set of modifiers.  If so, pause a short while.
                    // This enables fast switching between notes in the same octave, where no
                    // keychange is required.
                    &notemappings::Event::NoteMod(ref kopt) => {
                        let mut changes = 0;
                        let key_mods = vec![KbdKey::Shift, KbdKey::Control];
                        if let &Some(ref k) = kopt {
                            for key_mod in key_mods {
                                if &key_mod == k {
                                    if keygen.key_down(&key_mod) {
                                        changes = changes + 1;
                                    }
                                }
                                else {
                                    if keygen.key_up(&key_mod) {
                                        changes = changes + 1;
                                    }
                                }
                            }
                        }
                        else {
                            for key_mod in key_mods {
                                if keygen.key_up(&key_mod) {
                                    changes = changes + 1;
                                }
                            }
                        }
                        if changes > 0 {
                            thread::sleep(Duration::from_millis(OCTAVE_DELAY_MS));
                        }
                    },
                }
            }
        }
        else {
            println!("No note mapping for {:?} @ {:?}", msg.note(), msg.channel()); 
        }
    }

    /*
    let mut s = String::new();
    for &byte in raw_message {
        write!(&mut s, "{:X} ", byte).expect("Unable to write");
    }
    println!("Unhandled message for data: {}", s);
    */
}

fn generate_old_mappings(mappings: &mut NoteMappings) {
    let keys = vec![
        'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u', 'i'
    ];

    for (key_idx, key) in keys.iter().enumerate() {
        let base = MidiNote::C3.index();
        let mut note_mapping_lo = NoteMapping::new(MidiNote::new(key_idx as u8 + base).expect("Invalid note index"), 0, None);
        let mut note_mapping_mid = NoteMapping::new(MidiNote::new(key_idx as u8 + base + 12).expect("Invalid note index"), 0, None);
        let mut note_mapping_hi = NoteMapping::new(MidiNote::new(key_idx as u8 + base + 24).expect("Invalid note index"), 0, None);

        note_mapping_lo.on = NoteMapping::down_event(*key, Some(KbdKey::Control), Some(MOD_DELAY_MS));
        note_mapping_lo.off = NoteMapping::up_event(*key, Some(KbdKey::Control), Some(MOD_DELAY_MS));

        note_mapping_mid.on = NoteMapping::down_event(*key, None, None);
        note_mapping_mid.off = NoteMapping::up_event(*key, None, None);

        note_mapping_hi.on = NoteMapping::down_event(*key, Some(KbdKey::Shift), Some(MOD_DELAY_MS));
        note_mapping_hi.off = NoteMapping::up_event(*key, Some(KbdKey::Shift), Some(MOD_DELAY_MS));

        mappings.add(note_mapping_lo);
        mappings.add(note_mapping_mid);
        mappings.add(note_mapping_hi);
    }

    // Add pad buttons on the top of my keyboard, which are on channel 9.
    let pads = vec!['z', 'x', 'c', 'v', 'b', 'n', 'm', ','];
    for (pad_idx, pad) in pads.iter().enumerate() {
        let seq = vec![
            Event::KeyDown(KbdKey::Escape),
            Event::Delay(KEY_DELAY_MS),
            Event::KeyUp(KbdKey::Escape),

            Event::Delay(SYS_DELAY_MS),

            Event::KeyDown(KbdKey::Control),
            Event::KeyDown(KbdKey::Alt),
            Event::KeyDown(KbdKey::Shift),

            // Let the modifier keys get registered
            Event::Delay(MOD_DELAY_MS),

            Event::KeyDown(KbdKey::Layout(*pad)),
            Event::Delay(KEY_DELAY_MS),
            Event::KeyUp(KbdKey::Layout(*pad)),

            Event::Delay(MOD_DELAY_MS),
            Event::KeyUp(KbdKey::Shift),
            Event::KeyUp(KbdKey::Alt),
            Event::KeyUp(KbdKey::Control),
        ];

        let mut pad_mapping = NoteMapping::new(MidiNote::new(pad_idx as u8 + 40).expect("Invalid note index"), 9, None);
        pad_mapping.on = seq;
        mappings.add(pad_mapping);
    }

        /*
        if msg.channel() == 0 {
            if *msg.note() > MidiNote::C6 {
                println!("Note too high (max: C6)");
                return;
            } else if *msg.note() < MidiNote::C3 {
                println!("Note too low (min: C3)");
                return;
            }

            // Special case to deal with the high-C
            let note_idx = if *msg.note() == MidiNote::C6 {
                12
            } else {
                (msg.note().index() % 12) as usize
            };

            if *msg.event() == MidiEvent::NoteOn {
                // Hold Shift, since we're going up an octave
                if *msg.note() >= MidiNote::C5 {
                    println!("Sending Shift");
                    keygen.key_down(enigo::Key::Shift);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                } else if *msg.note() < MidiNote::C4 {
                    println!("Sending Control");
                    keygen.key_down(enigo::Key::Control);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                }

                println!("Sending key: {}", keys[note_idx]);
                keygen.key_down(enigo::Key::Layout(keys[note_idx]));
                thread::sleep(Duration::from_millis(KEY_DELAY_MS));
                keygen.key_up(enigo::Key::Layout(keys[note_idx]));

                if *msg.note() >= MidiNote::C5 {
                    keygen.key_up(enigo::Key::Shift);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                } else if *msg.note() >= MidiNote::C3 {
                    keygen.key_up(enigo::Key::Control);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                }
                return;
            } else if *msg.event() == MidiEvent::NoteOff {
                return;
            }
        }
        // Pad buttons on top
        else if msg.channel() == 9 {
            if msg.note().index() >= 40 && msg.note().index() <= 43 {
                if *msg.event() == MidiEvent::NoteOn {
                    let keys = vec!['z', 'x', 'c', 'v'];
                    let key_idx = ((msg.note().index() - 40) % 4) as usize;

                    println!("Switching instruments...");
                    keygen.key_down(enigo::Key::Escape);
                    thread::sleep(Duration::from_millis(KEY_DELAY_MS));
                    keygen.key_up(enigo::Key::Escape);

                    thread::sleep(Duration::from_millis(SYS_DELAY_MS));

                    keygen.key_down(enigo::Key::Control);
                    keygen.key_down(enigo::Key::Alt);
                    keygen.key_down(enigo::Key::Shift);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                    keygen.key_down(enigo::Key::Layout(keys[key_idx]));
                    thread::sleep(Duration::from_millis(KEY_DELAY_MS));
                    keygen.key_up(enigo::Key::Layout(keys[key_idx]));
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                    keygen.key_up(enigo::Key::Control);
                    keygen.key_up(enigo::Key::Alt);
                    keygen.key_up(enigo::Key::Shift);
                    return;
                } else if *msg.event() == MidiEvent::NoteOff {
                    return;
                }
            }
        }
        */
}

fn run(midi_name: Option<String>) -> Result<(), Box<Error>> {
    let mut target_device_name = midi_name.to_owned();
    let mut device_idx: Option<usize> = None;
    let mut connection: Option<MidiInputConnection<()>> = None;
    let app_state = AppState::new();

    //app_state.mappings().lock().unwrap().import("note_mappings.txt").ok();
    generate_old_mappings(&mut app_state.mappings().lock().unwrap());

    loop {
        let mut midi_in = MidiInput::new("perform")?;
        midi_in.ignore(Ignore::None);

        // If the index of the device has changed, reset the connection
        if let Some(idx) = device_idx {
            match midi_in.port_name(idx) {
                Err(_) => {
                    device_idx = None;
                    connection = None;
                }
                Ok(val) => {
                    if let Some(ref name) = target_device_name {
                        if &val != name {
                            device_idx = None;
                            connection = None;
                        }
                    }
                },
            }
        } else {
            device_idx = None;
            connection = None;
        };

        // If there is no connection, try to create a new one.
        if connection.is_none() {
            match target_device_name {
                None => println!("Connecting to first available device"),
                Some(ref s) => println!("Looking for device {}", s),
            }

            for i in 0..midi_in.port_count() {
                match midi_in.port_name(i) {
                    Err(_) => (),
                    Ok(name) => {
                        match target_device_name {
                            Some(ref s) => 
                                if &name == s {
                                    println!("Using device: {}", i);
                                    device_idx = Some(i);
                                },
                            None => {
                                println!("Using device: {}", i);
                                device_idx = Some(i);
                                target_device_name = Some(name);
                            },
                        }
                    }
                }
                println!("    {}", midi_in.port_name(i)?);
            }
        }

        if connection.is_none() {
            if let Some(idx) = device_idx {
                let app_state_thr = app_state.clone();
                match midi_in.connect(
                    idx,
                    "key monitor",
                    move |ts, raw_msg, _ignored| {
                        midi_callback(ts, raw_msg, &app_state_thr);
                    },
                    (),
                ) {
                    Err(reason) => println!("Unable to connect to device: {:?}", reason),
                    Ok(conn) => {
                        println!("Connection established");
                        connection = Some(conn);
                    }
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn list_devices() -> Result<(), Box<Error>> {
    let mut midi_in = MidiInput::new("perform")?;
    midi_in.ignore(Ignore::None);

    println!("Available MIDI devices:");
    for i in 0..midi_in.port_count() {
        println!("    {}", midi_in.port_name(i)?);
    }

    Ok(())
}
