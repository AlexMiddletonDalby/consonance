pub use crate::interval::Interval;
pub use crate::note::Note;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum ChordType {
    Maj,
    Min,
    Dim,
    Dom7,
    Maj7,
    Min7,
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
            ChordType::Dom7 => vec![
                Interval::Root,
                Interval::MajorThird,
                Interval::Fifth,
                Interval::MinorSeventh,
            ],
            ChordType::Maj7 => vec![
                Interval::Root,
                Interval::MajorThird,
                Interval::Fifth,
                Interval::MajorSeventh,
            ],
            ChordType::Min7 => vec![
                Interval::Root,
                Interval::MinorThird,
                Interval::Fifth,
                Interval::MinorSeventh,
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
                Self::Maj => "maj",
                Self::Min => "min",
                Self::Dim => "dim",
                Self::Dom7 => "7",
                Self::Maj7 => "maj7",
                Self::Min7 => "min7",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: Note,
    pub intervals: Vec<Interval>,
}
impl Chord {
    pub fn new(root: Note, intervals: &[Interval]) -> Self {
        Self {
            root,
            intervals: intervals.to_vec(),
        }
    }

    pub fn of_type(root: Note, chord_type: ChordType) -> Self {
        Self {
            root,
            intervals: chord_type.formula(),
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        let mut notes = Vec::new();

        for interval in &self.intervals {
            notes.push(self.root.transposed(interval));
        }

        return notes;
    }

    pub fn classify(&self) -> Option<ChordType> {
        for chord_type in enum_iterator::all::<ChordType>() {
            if self.intervals == chord_type.formula() {
                return Some(chord_type);
            }
        }

        return None;
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(chord_type) = self.classify() {
            write!(f, "{}{}", self.root, chord_type)
        } else {
            let mut interval_strings = Vec::new();
            for interval in &self.intervals {
                interval_strings.push(interval.to_string());
            }

            write!(f, "{} {}", self.root, interval_strings.join(" "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_notes_in_chords() {
        let cmaj = Chord::of_type(Note::C, ChordType::Maj);
        assert_eq!(cmaj.notes(), vec![Note::C, Note::E, Note::G]);

        let cmin = Chord::of_type(Note::C, ChordType::Min);
        assert_eq!(cmin.notes(), vec![Note::C, Note::DSharp, Note::G]);

        let amaj = Chord::of_type(Note::A, ChordType::Maj);
        assert_eq!(amaj.notes(), vec![Note::A, Note::CSharp, Note::E]);

        let fdim = Chord::of_type(Note::F, ChordType::Dim);
        assert_eq!(fdim.notes(), vec![Note::F, Note::GSharp, Note::B]);

        let e7 = Chord::of_type(Note::E, ChordType::Dom7);
        assert_eq!(e7.notes(), vec![Note::E, Note::GSharp, Note::B, Note::D]);

        let gmaj7= Chord::of_type(Note::G, ChordType::Maj7);
        assert_eq!(gmaj7.notes(), vec![Note::G, Note::B, Note::D, Note::FSharp]);

        let bmin7= Chord::of_type(Note::B, ChordType::Min7);
        assert_eq!(bmin7.notes(), vec![Note::B, Note::D, Note::FSharp, Note::A]);
    }

    #[test]
    fn display_chord_names() {
        let cmaj = Chord::of_type(Note::C, ChordType::Maj);
        assert_eq!(cmaj.to_string(), "Cmaj");

        let cmin = Chord::of_type(Note::C, ChordType::Min);
        assert_eq!(cmin.to_string(), "Cmin");

        let amaj = Chord::of_type(Note::A, ChordType::Maj);
        assert_eq!(amaj.to_string(), "Amaj");

        let fdim = Chord::of_type(Note::F, ChordType::Dim);
        assert_eq!(fdim.to_string(), "Fdim");

        let e7 = Chord::of_type(Note::E, ChordType::Dom7);
        assert_eq!(e7.to_string(), "E7");

        let gmaj7= Chord::of_type(Note::G, ChordType::Maj7);
        assert_eq!(gmaj7.to_string(), "Gmaj7");

        let bmin7= Chord::of_type(Note::B, ChordType::Min7);
        assert_eq!(bmin7.to_string(), "Bmin7");
    }
}
