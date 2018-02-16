#[derive(Debug, PartialEq)]
pub enum MidiEvent {
    NoteOn,
    NoteOff,
}

#[derive(Debug)]
pub struct MidiMessage {
    event: MidiEvent,
    channel: u8,
    note: u8,
    velocity: u8,
}

#[derive(Debug)]
pub enum MidiError {
    TooShort,
    Unimplemented(u8),
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
                    note: message[1] & 0x7f,
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
                    note: message[1] & 0x7f,
                    velocity: velocity,
                })
            },
            _ => Err(MidiError::Unimplemented(message[0])),
        }
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn note(&self) -> u8 {
        self.note
    }

    pub fn event(&self) -> &MidiEvent {
        &self.event
    }
}
