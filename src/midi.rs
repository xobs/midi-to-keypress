#[derive(Debug, PartialEq)]
pub enum MidiEvent {
    NoteOn,
    NoteOff,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum MidiNote {
    Cn = 0,
    Csn = 1,
    Dn = 2,
    Dsn = 3,
    En = 4,
    Fn = 5,
    Fsn = 6,
    Gn = 7,
    Gsn = 8,
    An = 9,
    Asn = 10,
    Bn = 11,
    C0 = 12,
    Cs0 = 13,
    D0 = 14,
    Ds0 = 15,
    E0 = 16,
    F0 = 17,
    Fs0 = 18,
    G0 = 19,
    Gs0 = 20,
    A0 = 21,
    As0 = 22,
    B0 = 23,
    C1 = 24,
    Cs1 = 25,
    D1 = 26,
    Ds1 = 27,
    E1 = 28,
    F1 = 29,
    Fs1 = 30,
    G1 = 31,
    Gs1 = 32,
    A1 = 33,
    As1 = 34,
    B1 = 35,
    C2 = 36,
    Cs2 = 37,
    D2 = 38,
    Ds2 = 39,
    E2 = 40,
    F2 = 41,
    Fs2 = 42,
    G2 = 43,
    Gs2 = 44,
    A2 = 45,
    As2 = 46,
    B2 = 47,
    C3 = 48,
    Cs3 = 49,
    D3 = 50,
    Ds3 = 51,
    E3 = 52,
    F3 = 53,
    Fs3 = 54,
    G3 = 55,
    Gs3 = 56,
    A3 = 57,
    As3 = 58,
    B3 = 59,
    C4 = 60,
    Cs4 = 61,
    D4 = 62,
    Ds4 = 63,
    E4 = 64,
    F4 = 65,
    Fs4 = 66,
    G4 = 67,
    Gs4 = 68,
    A4 = 69,
    As4 = 70,
    B4 = 71,
    C5 = 72,
    Cs5 = 73,
    D5 = 74,
    Ds5 = 75,
    E5 = 76,
    F5 = 77,
    Fs5 = 78,
    G5 = 79,
    Gs5 = 80,
    A5 = 81,
    As5 = 82,
    B5 = 83,
    C6 = 84,
    Cs6 = 85,
    D6 = 86,
    Ds6 = 87,
    E6 = 88,
    F6 = 89,
    Fs6 = 90,
    G6 = 91,
    Gs6 = 92,
    A6 = 93,
    As6 = 94,
    B6 = 95,
    C7 = 96,
    Cs7 = 97,
    D7 = 98,
    Ds7 = 99,
    E7 = 100,
    F7 = 101,
    Fs7 = 102,
    G7 = 103,
    Gs7 = 104,
    A7 = 105,
    As7 = 106,
    B7 = 107,
    C8 = 108,
    Cs8 = 109,
    D8 = 110,
    Ds8 = 111,
    E8 = 112,
    F8 = 113,
    Fs8 = 114,
    G8 = 115,
    Gs8 = 116,
    A8 = 117,
    As8 = 118,
    B8 = 119,
    C9 = 120,
    Cs9 = 121,
    D9 = 122,
    Ds9 = 123,
    E9 = 124,
    F9 = 125,
    Fs9 = 126,
    G9 = 127,
}

#[derive(Debug)]
pub struct MidiMessage {
    event: MidiEvent,
    channel: u8,
    note: MidiNote,
    velocity: u8,
}

#[derive(Debug)]
pub enum MidiError {
    TooShort,
    Unimplemented(u8),
    NoteOutOfRange,
}

impl MidiNote {
    pub fn new(val: u8) -> Result<MidiNote, MidiError> {
        if val > 127 {
            return Err(MidiError::NoteOutOfRange);
        }
        use std::mem;
        Ok(unsafe { mem::transmute(val)})
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }
}

impl MidiMessage {
    pub fn new(message: &[u8]) -> Result<MidiMessage, MidiError> {
        match message[0] & 0xf0 {
            0x80 => if message.len() < 3 {
                Err(MidiError::TooShort)
            } else {
                Ok(MidiMessage {
                    event: MidiEvent::NoteOff,
                    channel: message[0] & 0x0f,
                    note: MidiNote::new(message[1] & 0x7f)?,
                    velocity: message[2] & 0x7f,
                })
            },
            0x90 => if message.len() < 3 {
                Err(MidiError::TooShort)
            } else {
                let velocity = message[2] & 0x7f;
                let event = if velocity != 0 {
                    MidiEvent::NoteOn
                } else {
                    MidiEvent::NoteOff
                };
                Ok(MidiMessage {
                    event: event,
                    channel: message[0] & 0x0f,
                    note: MidiNote::new(message[1] & 0x7f)?,
                    velocity: velocity,
                })
            },
            _ => Err(MidiError::Unimplemented(message[0])),
        }
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note(&self) -> &MidiNote {
        &self.note
    }

    pub fn event(&self) -> &MidiEvent {
        &self.event
    }
}
