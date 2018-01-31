extern crate enigo;
extern crate midir;

use std::error::Error;
use std::time::Duration;
use std::thread;
use std::fmt::Write;

use enigo::KeyboardControllable;

use midir::{Ignore, MidiInput};

/// The amount of time to wait for a keyboard modifier to stick
const MOD_DELAY_MS: u64 = 5;

/// The amount of time to wait for a keydown event to stick
const KEY_DELAY_MS: u64 = 40;

/// The amount of time required for system events, such as Esc
const SYS_DELAY_MS: u64 = 200;

#[derive(Debug, PartialEq)]
enum MidiEvent {
    NoteOn,
    NoteOff,
}

#[derive(Debug)]
struct MidiMessage {
    event: MidiEvent,
    channel: u8,
    note: u8,
    velocity: u8,
}

#[derive(Debug)]
enum MidiError {
    TooShort,
    Unimplemented(u8),
}

fn parse_message(message: &[u8]) -> Result<MidiMessage, MidiError> {
    match message[0] & 0xf0 {
        0x80 => if message.len() < 3 {
            Err(MidiError::TooShort)
        } else {
            Ok(MidiMessage {
                event: MidiEvent::NoteOff,
                channel: message[0] & 0x0f,
                note: message[1] & 0x7f,
                velocity: message[2] & 0x7f,
            })
        },
        0x90 => if message.len() < 3 {
            Err(MidiError::TooShort)
        } else {
            Ok(MidiMessage {
                event: MidiEvent::NoteOn,
                channel: message[0] & 0x0f,
                note: message[1] & 0x7f,
                velocity: message[2] & 0x7f,
            })
        },
        _ => Err(MidiError::Unimplemented(message[0])),
    }
}

fn main() {
    list_devices().unwrap();
    run().unwrap();
}

fn midi_callback(_timestamp_us: u64, raw_message: &[u8], keygen: &mut enigo::Enigo) {
    let mut s = String::new();
    for &byte in raw_message {
        write!(&mut s, "{:X} ", byte).expect("Unable to write");
    }

    let keys = vec![
        'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u', 'i'
    ];

    println!("Got message for data: {}", s);
    if let Ok(msg) = parse_message(raw_message) {
        if msg.channel == 0 {
            if msg.note > 72 {
                println!("Note too high (max: C-6)");
                return;
            } else if msg.note < 36 {
                println!("Note too low (min: C-3)");
                return;
            }

            // Special case to deal with the high-C
            let note_idx = if msg.note == 72 {
                12
            } else {
                (msg.note % 12) as usize
            };

            if msg.event == MidiEvent::NoteOn {
                // Hold Shift, since we're going up an octave
                if msg.note >= 60 && msg.note <= 72 {
                    println!("Sending Shift");
                    keygen.key_down(enigo::Key::Shift);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                } else if msg.note >= 36 && msg.note <= 47 {
                    println!("Sending Control");
                    keygen.key_down(enigo::Key::Control);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                }

                println!("Sending key: {}", keys[note_idx]);
                keygen.key_down(enigo::Key::Layout(keys[note_idx]));
                thread::sleep(Duration::from_millis(KEY_DELAY_MS));
                keygen.key_up(enigo::Key::Layout(keys[note_idx]));

                if msg.note >= 60 && msg.note <= 72 {
                    keygen.key_up(enigo::Key::Shift);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                } else if msg.note >= 36 && msg.note <= 47 {
                    keygen.key_up(enigo::Key::Control);
                    thread::sleep(Duration::from_millis(MOD_DELAY_MS));
                }
            }
        }
        // Pad buttons on top
        else if msg.channel == 9 {
            if msg.event == MidiEvent::NoteOn && msg.note >= 40 && msg.note <= 43 {
                let keys = vec!['z', 'x', 'c', 'v'];
                let key_idx = ((msg.note - 40) % 4) as usize;

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
            }
        }

        println!("Parsed Message: {:?}", msg);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut keygen = enigo::Enigo::new();
    let mut midi_in = MidiInput::new("keyboard-tweak")?;
    midi_in.ignore(Ignore::None);

    println!("Connecting to {}", midi_in.port_name(0)?);
    let _connection = midi_in.connect(
        0,
        "key monitor",
        move |ts, raw_msg, _ignored| {
            midi_callback(ts, raw_msg, &mut keygen);
        },
        (),
    );

    loop {
        thread::sleep(Duration::from_secs(1));
    }
    /*qwwet
    let midi_out = MidiOutput::new("midir test output")?;

    let mut input = String::new();

    loop {
        println!("Available input ports:");
        for i in 0..midi_in.port_count() {
            println!("{}: {}", i, midi_in.port_name(i)?);
        }
        
        println!("\nAvailable output ports:");
        for i in 0..midi_out.port_count() {
            println!("{}: {}", i, midi_out.port_name(i)?);
        }

        // run in endless loop if "--loop" parameter is specified
        match ::std::env::args().nth(1) {
            Some(ref arg) if arg == "--loop" => {}
            _ => break
        }
        print!("\nPress <enter> to retry ...");
        stdout().flush()?;
        input.clear();
        stdin().read_line(&mut input)?;
        println!("\n");
    }
    */
}

fn list_devices() -> Result<(), Box<Error>> {
    let mut midi_in = MidiInput::new("keyboard-tweak")?;
    midi_in.ignore(Ignore::None);

    println!("Available input ports:");
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }

    Ok(())
}
