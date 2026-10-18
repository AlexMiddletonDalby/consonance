use crate::chord::Chord;
use crate::note::Note;
use crate::octave::Octave;

const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    data: Vec<u8>,
}
impl Message {
    fn midi_note_value(note: &Note, octave: &Octave) -> u8 {
        let octave_shift = octave.relative_to_middle_c() as i16;
        let base_note = match note {
            Note::C => 60,
            Note::CSharp => 61,
            Note::D => 62,
            Note::DSharp => 63,
            Note::E => 64,
            Note::F => 65,
            Note::FSharp => 66,
            Note::G => 67,
            Note::GSharp => 68,
            Note::A => 69,
            Note::ASharp => 70,
            Note::B => 71,
        };
        (base_note + (12 * octave_shift)).clamp(0, 127) as u8
    }

    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    fn note(note: &Note, message: u8, props: &Properties) -> Self {
        Self {
            data: vec![
                message,
                Self::midi_note_value(&note, &props.octave),
                props.velocity,
            ],
        }
    }

    fn combined(messages: &[Self]) -> Self {
        let mut data: Vec<u8> = Vec::new();
        for message in messages.into_iter() {
            data.extend(&message.data);
        }

        Self { data }
    }
}
#[derive(Clone)]
pub struct Properties {
    pub octave: Octave,
    pub velocity: u8,
}

pub trait ToMessage {
    fn note_message(&self, message: u8, props: Properties) -> Message;

    fn note_on(&self, props: Properties) -> Message {
        self.note_message(NOTE_ON, props)
    }
    fn note_off(&self, props: Properties) -> Message {
        self.note_message(NOTE_OFF, props)
    }
}
impl ToMessage for Note {
    fn note_message(&self, message: u8, props: Properties) -> Message {
        Message::note(self, message, &props)
    }
}
impl ToMessage for Chord {
    fn note_message(&self, message: u8, props: Properties) -> Message {
        let mut messages: Vec<Message> = Vec::new();
        let mut props = props.clone();

        let mut previous_note: Option<Note> = None;
        for note in self.notes() {
            if let Some(previous) = previous_note {
                let wrapped_around = note < previous;
                if wrapped_around {
                    props.octave = props.octave.plus(1);
                }
            }

            messages.push(Message::note(&note, message, &props));
            previous_note = Some(note);
        }

        Message::combined(&messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Extension, Modifiers, Triad};

    #[test]
    fn build_note_messages_from_notes() {
        assert_eq!(
            Message::note(
                &Note::C,
                NOTE_ON,
                &Properties {
                    octave: Octave::middle_c(),
                    velocity: 127
                }
            ),
            Message::new(&[0x90, 60, 127])
        );
        assert_eq!(
            Message::note(
                &Note::F,
                NOTE_ON,
                &Properties {
                    octave: Octave::middle_c().plus(1),
                    velocity: 100
                }
            ),
            Message::new(&[0x90, 77, 100])
        );
        assert_eq!(
            Message::note(
                &Note::A,
                NOTE_ON,
                &Properties {
                    octave: Octave::build(-12),
                    velocity: 0
                }
            ),
            Message::new(&[0x90, 0, 0])
        );
        assert_eq!(
            Message::note(
                &Note::A,
                NOTE_ON,
                &Properties {
                    octave: Octave::build(64),
                    velocity: 50
                }
            ),
            Message::new(&[0x90, 127, 50])
        );
        assert_eq!(
            Message::note(
                &Note::C,
                NOTE_OFF,
                &Properties {
                    octave: Octave::middle_c(),
                    velocity: 127
                }
            ),
            Message::new(&[0x80, 60, 127])
        );
        assert_eq!(
            Message::note(
                &Note::F,
                NOTE_OFF,
                &Properties {
                    octave: Octave::middle_c().plus(1),
                    velocity: 100
                }
            ),
            Message::new(&[0x80, 77, 100])
        );
        assert_eq!(
            Message::note(
                &Note::A,
                NOTE_OFF,
                &Properties {
                    octave: Octave::build(-12),
                    velocity: 0
                }
            ),
            Message::new(&[0x80, 0, 0])
        );
        assert_eq!(
            Message::note(
                &Note::A,
                NOTE_OFF,
                &Properties {
                    octave: Octave::build(64),
                    velocity: 50
                }
            ),
            Message::new(&[0x80, 127, 50])
        );
    }

    #[test]
    fn combine_note_messages_together() {
        let first = Message::new(&[0x90, 60, 127]);
        let second = Message::new(&[0x90, 65, 100]);

        assert_eq!(
            Message::combined(&[first, second]),
            Message::new(&[0x90, 60, 127, 0x90, 65, 100])
        );
    }

    #[test]
    fn convert_notes_to_note_messages() {
        let c4_on_midi = Note::C.note_on(Properties {
            octave: Octave::middle_c(),
            velocity: 127,
        });
        assert_eq!(c4_on_midi, Message::new(&[0x90, 60, 127]));

        let f5_on_midi = Note::F.note_off(Properties {
            octave: Octave::middle_c().plus(1),
            velocity: 100,
        });
        assert_eq!(f5_on_midi, Message::new(&[0x80, 77, 100]));
    }

    #[test]
    fn convert_chords_to_note_messages() {
        let cmaj4_on_midi = Chord::triad(Note::C, Triad::Maj).note_on(Properties {
            octave: Octave::middle_c(),
            velocity: 127,
        });
        assert_eq!(
            cmaj4_on_midi,
            Message::new(&[0x90, 60, 127, 0x90, 64, 127, 0x90, 67, 127])
        );

        let amin7_off_midi = Chord::build(
            Note::A,
            Triad::Min,
            Modifiers {
                extension: Some(Extension::MinorSeventh),
            },
        )
        .note_off(Properties {
            octave: Octave::middle_c(),
            velocity: 127,
        });
        assert_eq!(
            amin7_off_midi,
            Message::new(&[0x80, 69, 127, 0x80, 72, 127, 0x80, 76, 127, 0x80, 79, 127])
        );
    }
}
