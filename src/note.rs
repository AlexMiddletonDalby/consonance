pub use crate::interval::Interval;
use std::fmt;

type MidiNoteMessage = [u8; 3];
pub struct MidiNoteProperties {
    pub octave: i16,
    pub velocity: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}
impl Note {
    pub fn transposed(&self, interval: &Interval) -> Note {
        let mut note = self.clone();
        let semitones = interval.semitones();

        for _ in 0..semitones {
            note = enum_iterator::next_cycle(&note);
        }

        return note;
    }

    fn midi_note_message(&self, message_type: u8, props: MidiNoteProperties) -> MidiNoteMessage {
        const MIDDLE_C_OCTAVE: i16 = 4;
        let octave_shift = props.octave - MIDDLE_C_OCTAVE;
        let note = match self {
            Self::C => 60,
            Self::CSharp => 61,
            Self::D => 62,
            Self::DSharp => 63,
            Self::E => 64,
            Self::F => 65,
            Self::FSharp => 66,
            Self::G => 67,
            Self::GSharp => 68,
            Self::A => 69,
            Self::ASharp => 70,
            Self::B => 71,
        } + (12 * octave_shift);

        let note = note.clamp(0, 127) as u8;

        [message_type, note, props.velocity]
    }

    pub fn note_on(&self, props: MidiNoteProperties) -> MidiNoteMessage {
        self.midi_note_message(0x90, props)
    }

    pub fn note_off(&self, props: MidiNoteProperties) -> MidiNoteMessage {
        self.midi_note_message(0x80, props)
    }
}
impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::C => "C",
                Self::CSharp => "C#",
                Self::D => "D",
                Self::DSharp => "D#",
                Self::E => "E",
                Self::F => "F",
                Self::FSharp => "F#",
                Self::G => "G",
                Self::GSharp => "G#",
                Self::A => "A",
                Self::ASharp => "A#",
                Self::B => "B",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_notes() {
        assert_eq!(Note::C.transposed(&Interval::Fifth), Note::G);
        assert_eq!(Note::G.transposed(&Interval::Octave), Note::G);
        assert_eq!(Note::E.transposed(&Interval::MinorSecond), Note::F);
    }

    #[test]
    fn convert_notes_to_midi() {
        assert_eq!(
            Note::C.note_on(MidiNoteProperties {
                octave: 4,
                velocity: 127
            }),
            [0x90, 60, 127]
        );
        assert_eq!(
            Note::F.note_on(MidiNoteProperties {
                octave: 5,
                velocity: 100
            }),
            [0x90, 77, 100]
        );
        assert_eq!(
            Note::A.note_on(MidiNoteProperties {
                octave: -12,
                velocity: 0
            }),
            [0x90, 0, 0]
        );
        assert_eq!(
            Note::C.note_on(MidiNoteProperties {
                octave: 64,
                velocity: 50
            }),
            [0x90, 127, 50]
        );
        assert_eq!(
            Note::C.note_off(MidiNoteProperties {
                octave: 4,
                velocity: 127
            }),
            [0x80, 60, 127]
        );
        assert_eq!(
            Note::F.note_off(MidiNoteProperties {
                octave: 5,
                velocity: 100
            }),
            [0x80, 77, 100]
        );
        assert_eq!(
            Note::A.note_off(MidiNoteProperties {
                octave: -12,
                velocity: 0
            }),
            [0x80, 0, 0]
        );
        assert_eq!(
            Note::C.note_off(MidiNoteProperties {
                octave: 64,
                velocity: 50
            }),
            [0x80, 127, 50]
        );
    }
}
