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
    Unparseable,
}

impl MidiNote {
    pub fn new(val: u8) -> Result<MidiNote, MidiError> {
        if val > 127 {
            return Err(MidiError::NoteOutOfRange);
        }
        use std::mem;
        Ok(unsafe { mem::transmute(val) })
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn new_from_text(txt: &str) -> Result<MidiNote, MidiError> {
        let s = txt.to_lowercase();
        if s.starts_with('#') {
            Err(MidiError::Unparseable)
        } else if s.starts_with("cn") {
            Ok(MidiNote::Cn)
        } else if s.starts_with("csn") {
            Ok(MidiNote::Csn)
        } else if s.starts_with("dn") {
            Ok(MidiNote::Dn)
        } else if s.starts_with("dsn") {
            Ok(MidiNote::Dsn)
        } else if s.starts_with("en") {
            Ok(MidiNote::En)
        } else if s.starts_with("fn") {
            Ok(MidiNote::Fn)
        } else if s.starts_with("fsn") {
            Ok(MidiNote::Fsn)
        } else if s.starts_with("gn") {
            Ok(MidiNote::Gn)
        } else if s.starts_with("gsn") {
            Ok(MidiNote::Gsn)
        } else if s.starts_with("an") {
            Ok(MidiNote::An)
        } else if s.starts_with("asn") {
            Ok(MidiNote::Asn)
        } else if s.starts_with("bn") {
            Ok(MidiNote::Bn)
        } else if s.starts_with("c0") {
            Ok(MidiNote::C0)
        } else if s.starts_with("cs0") {
            Ok(MidiNote::Cs0)
        } else if s.starts_with("d0") {
            Ok(MidiNote::D0)
        } else if s.starts_with("ds0") {
            Ok(MidiNote::Ds0)
        } else if s.starts_with("e0") {
            Ok(MidiNote::E0)
        } else if s.starts_with("f0") {
            Ok(MidiNote::F0)
        } else if s.starts_with("fs0") {
            Ok(MidiNote::Fs0)
        } else if s.starts_with("g0") {
            Ok(MidiNote::G0)
        } else if s.starts_with("gs0") {
            Ok(MidiNote::Gs0)
        } else if s.starts_with("a0") {
            Ok(MidiNote::A0)
        } else if s.starts_with("as0") {
            Ok(MidiNote::As0)
        } else if s.starts_with("b0") {
            Ok(MidiNote::B0)
        } else if s.starts_with("c1") {
            Ok(MidiNote::C1)
        } else if s.starts_with("cs1") {
            Ok(MidiNote::Cs1)
        } else if s.starts_with("d1") {
            Ok(MidiNote::D1)
        } else if s.starts_with("ds1") {
            Ok(MidiNote::Ds1)
        } else if s.starts_with("e1") {
            Ok(MidiNote::E1)
        } else if s.starts_with("f1") {
            Ok(MidiNote::F1)
        } else if s.starts_with("fs1") {
            Ok(MidiNote::Fs1)
        } else if s.starts_with("g1") {
            Ok(MidiNote::G1)
        } else if s.starts_with("gs1") {
            Ok(MidiNote::Gs1)
        } else if s.starts_with("a1") {
            Ok(MidiNote::A1)
        } else if s.starts_with("as1") {
            Ok(MidiNote::As1)
        } else if s.starts_with("b1") {
            Ok(MidiNote::B1)
        } else if s.starts_with("c2") {
            Ok(MidiNote::C2)
        } else if s.starts_with("cs2") {
            Ok(MidiNote::Cs2)
        } else if s.starts_with("d2") {
            Ok(MidiNote::D2)
        } else if s.starts_with("ds2") {
            Ok(MidiNote::Ds2)
        } else if s.starts_with("e2") {
            Ok(MidiNote::E2)
        } else if s.starts_with("f2") {
            Ok(MidiNote::F2)
        } else if s.starts_with("fs2") {
            Ok(MidiNote::Fs2)
        } else if s.starts_with("g2") {
            Ok(MidiNote::G2)
        } else if s.starts_with("gs2") {
            Ok(MidiNote::Gs2)
        } else if s.starts_with("a2") {
            Ok(MidiNote::A2)
        } else if s.starts_with("as2") {
            Ok(MidiNote::As2)
        } else if s.starts_with("b2") {
            Ok(MidiNote::B2)
        } else if s.starts_with("c3") {
            Ok(MidiNote::C3)
        } else if s.starts_with("cs3") {
            Ok(MidiNote::Cs3)
        } else if s.starts_with("d3") {
            Ok(MidiNote::D3)
        } else if s.starts_with("ds3") {
            Ok(MidiNote::Ds3)
        } else if s.starts_with("e3") {
            Ok(MidiNote::E3)
        } else if s.starts_with("f3") {
            Ok(MidiNote::F3)
        } else if s.starts_with("fs3") {
            Ok(MidiNote::Fs3)
        } else if s.starts_with("g3") {
            Ok(MidiNote::G3)
        } else if s.starts_with("gs3") {
            Ok(MidiNote::Gs3)
        } else if s.starts_with("a3") {
            Ok(MidiNote::A3)
        } else if s.starts_with("as3") {
            Ok(MidiNote::As3)
        } else if s.starts_with("b3") {
            Ok(MidiNote::B3)
        } else if s.starts_with("c4") {
            Ok(MidiNote::C4)
        } else if s.starts_with("cs4") {
            Ok(MidiNote::Cs4)
        } else if s.starts_with("d4") {
            Ok(MidiNote::D4)
        } else if s.starts_with("ds4") {
            Ok(MidiNote::Ds4)
        } else if s.starts_with("e4") {
            Ok(MidiNote::E4)
        } else if s.starts_with("f4") {
            Ok(MidiNote::F4)
        } else if s.starts_with("fs4") {
            Ok(MidiNote::Fs4)
        } else if s.starts_with("g4") {
            Ok(MidiNote::G4)
        } else if s.starts_with("gs4") {
            Ok(MidiNote::Gs4)
        } else if s.starts_with("a4") {
            Ok(MidiNote::A4)
        } else if s.starts_with("as4") {
            Ok(MidiNote::As4)
        } else if s.starts_with("b4") {
            Ok(MidiNote::B4)
        } else if s.starts_with("c5") {
            Ok(MidiNote::C5)
        } else if s.starts_with("cs5") {
            Ok(MidiNote::Cs5)
        } else if s.starts_with("d5") {
            Ok(MidiNote::D5)
        } else if s.starts_with("ds5") {
            Ok(MidiNote::Ds5)
        } else if s.starts_with("e5") {
            Ok(MidiNote::E5)
        } else if s.starts_with("f5") {
            Ok(MidiNote::F5)
        } else if s.starts_with("fs5") {
            Ok(MidiNote::Fs5)
        } else if s.starts_with("g5") {
            Ok(MidiNote::G5)
        } else if s.starts_with("gs5") {
            Ok(MidiNote::Gs5)
        } else if s.starts_with("a5") {
            Ok(MidiNote::A5)
        } else if s.starts_with("as5") {
            Ok(MidiNote::As5)
        } else if s.starts_with("b5") {
            Ok(MidiNote::B5)
        } else if s.starts_with("c6") {
            Ok(MidiNote::C6)
        } else if s.starts_with("cs6") {
            Ok(MidiNote::Cs6)
        } else if s.starts_with("d6") {
            Ok(MidiNote::D6)
        } else if s.starts_with("ds6") {
            Ok(MidiNote::Ds6)
        } else if s.starts_with("e6") {
            Ok(MidiNote::E6)
        } else if s.starts_with("f6") {
            Ok(MidiNote::F6)
        } else if s.starts_with("fs6") {
            Ok(MidiNote::Fs6)
        } else if s.starts_with("g6") {
            Ok(MidiNote::G6)
        } else if s.starts_with("gs6") {
            Ok(MidiNote::Gs6)
        } else if s.starts_with("a6") {
            Ok(MidiNote::A6)
        } else if s.starts_with("as6") {
            Ok(MidiNote::As6)
        } else if s.starts_with("b6") {
            Ok(MidiNote::B6)
        } else if s.starts_with("c7") {
            Ok(MidiNote::C7)
        } else if s.starts_with("cs7") {
            Ok(MidiNote::Cs7)
        } else if s.starts_with("d7") {
            Ok(MidiNote::D7)
        } else if s.starts_with("ds7") {
            Ok(MidiNote::Ds7)
        } else if s.starts_with("e7") {
            Ok(MidiNote::E7)
        } else if s.starts_with("f7") {
            Ok(MidiNote::F7)
        } else if s.starts_with("fs7") {
            Ok(MidiNote::Fs7)
        } else if s.starts_with("g7") {
            Ok(MidiNote::G7)
        } else if s.starts_with("gs7") {
            Ok(MidiNote::Gs7)
        } else if s.starts_with("a7") {
            Ok(MidiNote::A7)
        } else if s.starts_with("as7") {
            Ok(MidiNote::As7)
        } else if s.starts_with("b7") {
            Ok(MidiNote::B7)
        } else if s.starts_with("c8") {
            Ok(MidiNote::C8)
        } else if s.starts_with("cs8") {
            Ok(MidiNote::Cs8)
        } else if s.starts_with("d8") {
            Ok(MidiNote::D8)
        } else if s.starts_with("ds8") {
            Ok(MidiNote::Ds8)
        } else if s.starts_with("e8") {
            Ok(MidiNote::E8)
        } else if s.starts_with("f8") {
            Ok(MidiNote::F8)
        } else if s.starts_with("fs8") {
            Ok(MidiNote::Fs8)
        } else if s.starts_with("g8") {
            Ok(MidiNote::G8)
        } else if s.starts_with("gs8") {
            Ok(MidiNote::Gs8)
        } else if s.starts_with("a8") {
            Ok(MidiNote::A8)
        } else if s.starts_with("as8") {
            Ok(MidiNote::As8)
        } else if s.starts_with("b8") {
            Ok(MidiNote::B8)
        } else if s.starts_with("c9") {
            Ok(MidiNote::C9)
        } else if s.starts_with("cs9") {
            Ok(MidiNote::Cs9)
        } else if s.starts_with("d9") {
            Ok(MidiNote::D9)
        } else if s.starts_with("ds9") {
            Ok(MidiNote::Ds9)
        } else if s.starts_with("e9") {
            Ok(MidiNote::E9)
        } else if s.starts_with("f9") {
            Ok(MidiNote::F9)
        } else if s.starts_with("fs9") {
            Ok(MidiNote::Fs9)
        } else if s.starts_with("g9") {
            Ok(MidiNote::G9)
        } else {
            Err(MidiError::Unparseable)
        }
    }

    pub fn index(self) -> u8 {
        self as u8
    }
}

impl MidiMessage {
    pub fn new(message: &[u8]) -> Result<MidiMessage, MidiError> {
        match message[0] & 0xf0 {
            0x80 => {
                if message.len() < 3 {
                    Err(MidiError::TooShort)
                } else {
                    Ok(MidiMessage {
                        event: MidiEvent::NoteOff,
                        channel: message[0] & 0x0f,
                        note: MidiNote::new(message[1] & 0x7f)?,
                        velocity: message[2] & 0x7f,
                    })
                }
            }
            0x90 => {
                if message.len() < 3 {
                    Err(MidiError::TooShort)
                } else {
                    let velocity = message[2] & 0x7f;
                    let event = if velocity != 0 {
                        MidiEvent::NoteOn
                    } else {
                        MidiEvent::NoteOff
                    };
                    Ok(MidiMessage {
                        event,
                        channel: message[0] & 0x0f,
                        note: MidiNote::new(message[1] & 0x7f)?,
                        velocity,
                    })
                }
            }
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
