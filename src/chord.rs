pub use crate::interval::Interval;
pub use crate::note::Note;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ChordType {
    Maj,
    Min,
    Dim,
}
impl ChordType {
    fn formula(&self) -> Vec<Interval> {
        match self {
            ChordType::Maj => {
                vec![Interval::Root, Interval::MajorThird, Interval::Fifth]
            }
            ChordType::Min => {
                vec![Interval::Root, Interval::MinorThird, Interval::Fifth]
            }
            ChordType::Dim => vec![
                Interval::Root,
                Interval::MinorThird,
                Interval::DiminishedFifth,
            ],
        }
    }
}
impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Maj => "Major",
                Self::Min => "Minor",
                Self::Dim => "Diminished",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    pub root: Note,
    pub chord_type: ChordType,
}
impl Chord {
    pub fn new(root: Note, chord_type: ChordType) -> Self {
        Self { root, chord_type }
    }

    pub fn notes(&self) -> Vec<Note> {
        let mut notes = Vec::new();

        for interval in self.chord_type.formula() {
            notes.push(self.root.transposed(interval));
        }

        return notes;
    }
}
impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.root, self.chord_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_notes_in_chords() {
        let cmaj = Chord::new(Note::C, ChordType::Maj);
        assert_eq!(cmaj.notes(), vec![Note::C, Note::E, Note::G]);

        let cmin = Chord::new(Note::C, ChordType::Min);
        assert_eq!(cmin.notes(), vec![Note::C, Note::DSharp, Note::G]);

        let amaj = Chord::new(Note::A, ChordType::Maj);
        assert_eq!(amaj.notes(), vec![Note::A, Note::CSharp, Note::E]);

        let fdim = Chord::new(Note::F, ChordType::Dim);
        assert_eq!(fdim.notes(), vec![Note::F, Note::GSharp, Note::B]);
    }

    #[test]
    fn display_chord_names() {
        let cmaj = Chord::new(Note::C, ChordType::Maj);
        assert_eq!(cmaj.to_string(), "C Major");

        let cmin = Chord::new(Note::C, ChordType::Min);
        assert_eq!(cmin.to_string(), "C Minor");

        let amaj = Chord::new(Note::A, ChordType::Maj);
        assert_eq!(amaj.to_string(), "A Major");

        let fdim = Chord::new(Note::F, ChordType::Dim);
        assert_eq!(fdim.to_string(), "F Diminished");
    }
}
