use crate::chord::Chord;
use crate::note::Note;

const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;

pub trait ToMidi<M> {
    fn to_midi(&self, message: u8, props: Properties) -> M;

    fn note_on(&self, props: Properties) -> M {
        self.to_midi(NOTE_ON, props)
    }
    fn note_off(&self, props: Properties) -> M {
        self.to_midi(NOTE_OFF, props)
    }
}

type Message = [u8; 3];

#[derive(Clone)]
pub struct Properties {
    pub octave: i16,
    pub velocity: u8,
}

fn midi_note_value(note: &Note, octave: i16) -> u8 {
    const MIDDLE_C_OCTAVE: i16 = 4;
    let octave_shift = octave - MIDDLE_C_OCTAVE;
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

fn midi_message(note: &Note, message_type: u8, props: &Properties) -> Message {
    [
        message_type,
        midi_note_value(note, props.octave),
        props.velocity,
    ]
}

impl ToMidi<Message> for Note {
    fn to_midi(&self, message: u8, props: Properties) -> Message {
        midi_message(self, message, &props)
    }
}

impl ToMidi<Vec<Message>> for Chord {
    fn to_midi(&self, message: u8, props: Properties) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        let mut props = props.clone();

        let mut previous_note: Option<Note> = None;
        for note in self.notes() {
            if let Some(previous) = previous_note {
                let wrapped_around = note < previous;
                if wrapped_around {
                    props.octave += 1;
                }
            }

            messages.push(midi_message(&note, message, &props));
            previous_note = Some(note);
        }

        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Extension, Modifiers, Triad};

    #[test]
    fn convert_single_notes_to_midi_messages() {
        assert_eq!(
            midi_message(
                &Note::C,
                NOTE_ON,
                &Properties {
                    octave: 4,
                    velocity: 127
                }
            ),
            [0x90, 60, 127]
        );
        assert_eq!(
            midi_message(
                &Note::F,
                NOTE_ON,
                &Properties {
                    octave: 5,
                    velocity: 100
                }
            ),
            [0x90, 77, 100]
        );
        assert_eq!(
            midi_message(
                &Note::A,
                NOTE_ON,
                &Properties {
                    octave: -12,
                    velocity: 0
                }
            ),
            [0x90, 0, 0]
        );
        assert_eq!(
            midi_message(
                &Note::A,
                NOTE_ON,
                &Properties {
                    octave: 64,
                    velocity: 50
                }
            ),
            [0x90, 127, 50]
        );
        assert_eq!(
            midi_message(
                &Note::C,
                NOTE_OFF,
                &Properties {
                    octave: 4,
                    velocity: 127
                }
            ),
            [0x80, 60, 127]
        );
        assert_eq!(
            midi_message(
                &Note::F,
                NOTE_OFF,
                &Properties {
                    octave: 5,
                    velocity: 100
                }
            ),
            [0x80, 77, 100]
        );
        assert_eq!(
            midi_message(
                &Note::A,
                NOTE_OFF,
                &Properties {
                    octave: -12,
                    velocity: 0
                }
            ),
            [0x80, 0, 0]
        );
        assert_eq!(
            midi_message(
                &Note::A,
                NOTE_OFF,
                &Properties {
                    octave: 64,
                    velocity: 50
                }
            ),
            [0x80, 127, 50]
        );
    }

    #[test]
    fn convert_notes_to_midi_stream() {
        let c4_on_midi = Note::C.note_on(Properties {
            octave: 4,
            velocity: 127,
        });
        assert_eq!(c4_on_midi, [0x90, 60, 127]);

        let f5_on_midi = Note::F.note_off(Properties {
            octave: 5,
            velocity: 100,
        });
        assert_eq!(f5_on_midi, [0x80, 77, 100]);
    }

    #[test]
    fn convert_chords_to_midi_stream() {
        let cmaj4_on_midi = Chord::triad(Note::C, Triad::Maj).note_on(Properties {
            octave: 4,
            velocity: 127,
        });
        assert_eq!(
            cmaj4_on_midi,
            vec![[0x90, 60, 127], [0x90, 64, 127], [0x90, 67, 127]]
        );

        let amin7_off_midi = Chord::build(
            Note::A,
            Triad::Min,
            Modifiers {
                extension: Some(Extension::MinorSeventh),
            },
        )
        .note_off(Properties {
            octave: 4,
            velocity: 127,
        });
        assert_eq!(
            amin7_off_midi,
            vec![
                [0x80, 69, 127],
                [0x80, 72, 127],
                [0x80, 76, 127],
                [0x80, 79, 127]
            ]
        );
    }
}
