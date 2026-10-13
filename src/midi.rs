use crate::chord::Chord;
use crate::note::Note;

const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;

pub trait Midi {
    fn midi_message(&self, message: u8, props: NoteProperties)
    -> impl Iterator<Item = NoteMessage>;

    fn note_on(&self, props: NoteProperties) -> impl Iterator<Item = NoteMessage> {
        self.midi_message(NOTE_ON, props)
    }
    fn note_off(&self, props: NoteProperties) -> impl Iterator<Item = NoteMessage> {
        self.midi_message(NOTE_OFF, props)
    }
}

type NoteMessage = [u8; 3];
#[derive(Clone)]
pub struct NoteProperties {
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

fn to_midi_message(note: &Note, message_type: u8, props: &NoteProperties) -> NoteMessage {
    [
        message_type,
        midi_note_value(note, props.octave),
        props.velocity,
    ]
}

impl Midi for Note {
    fn midi_message(
        &self,
        message: u8,
        props: NoteProperties,
    ) -> impl Iterator<Item = NoteMessage> {
        [to_midi_message(self, message, &props)].into_iter()
    }
}

impl Midi for Chord {
    fn midi_message(
        &self,
        message: u8,
        props: NoteProperties,
    ) -> impl Iterator<Item = NoteMessage> {
        let mut messages: Vec<NoteMessage> = Vec::new();
        let mut props = props.clone();

        let mut previous_note: Option<Note> = None;
        for note in self.notes() {
            if let Some(previous) = previous_note {
                let wrapped_around = note < previous;
                if wrapped_around {
                    props.octave += 1;
                }
            }

            messages.push(to_midi_message(&note, message, &props));
            previous_note = Some(note);
        }

        messages.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Extension, Modifiers, Triad};

    #[test]
    fn convert_single_notes_to_midi() {
        assert_eq!(
            to_midi_message(
                &Note::C,
                NOTE_ON,
                &NoteProperties {
                    octave: 4,
                    velocity: 127
                }
            ),
            [0x90, 60, 127]
        );
        assert_eq!(
            to_midi_message(
                &Note::F,
                NOTE_ON,
                &NoteProperties {
                    octave: 5,
                    velocity: 100
                }
            ),
            [0x90, 77, 100]
        );
        assert_eq!(
            to_midi_message(
                &Note::A,
                NOTE_ON,
                &NoteProperties {
                    octave: -12,
                    velocity: 0
                }
            ),
            [0x90, 0, 0]
        );
        assert_eq!(
            to_midi_message(
                &Note::A,
                NOTE_ON,
                &NoteProperties {
                    octave: 64,
                    velocity: 50
                }
            ),
            [0x90, 127, 50]
        );
        assert_eq!(
            to_midi_message(
                &Note::C,
                NOTE_OFF,
                &NoteProperties {
                    octave: 4,
                    velocity: 127
                }
            ),
            [0x80, 60, 127]
        );
        assert_eq!(
            to_midi_message(
                &Note::F,
                NOTE_OFF,
                &NoteProperties {
                    octave: 5,
                    velocity: 100
                }
            ),
            [0x80, 77, 100]
        );
        assert_eq!(
            to_midi_message(
                &Note::A,
                NOTE_OFF,
                &NoteProperties {
                    octave: -12,
                    velocity: 0
                }
            ),
            [0x80, 0, 0]
        );
        assert_eq!(
            to_midi_message(
                &Note::A,
                NOTE_OFF,
                &NoteProperties {
                    octave: 64,
                    velocity: 50
                }
            ),
            [0x80, 127, 50]
        );
    }

    #[test]
    fn convert_notes_to_midi_stream() {
        let c4_on_midi: Vec<NoteMessage> = Note::C
            .note_on(NoteProperties {
                octave: 4,
                velocity: 127,
            })
            .collect();
        assert_eq!(c4_on_midi, vec![[0x90, 60, 127]]);

        let f5_on_midi: Vec<NoteMessage> = Note::F
            .note_off(NoteProperties {
                octave: 5,
                velocity: 100,
            })
            .collect();
        assert_eq!(f5_on_midi, vec![[0x80, 77, 100]]);
    }

    #[test]
    fn convert_chords_to_midi_stream() {
        let cmaj4_on_midi: Vec<NoteMessage> = Chord::triad(Note::C, Triad::Maj)
            .note_on(NoteProperties {
                octave: 4,
                velocity: 127,
            })
            .collect();

        assert_eq!(
            cmaj4_on_midi,
            vec![[0x90, 60, 127], [0x90, 64, 127], [0x90, 67, 127]]
        );

        let amin7_off_midi: Vec<NoteMessage> = Chord::build(
            Note::A,
            Triad::Min,
            Modifiers {
                extension: Some(Extension::MinorSeventh),
            },
        )
        .note_off(NoteProperties {
            octave: 4,
            velocity: 127,
        })
        .collect();

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
