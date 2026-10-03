use crate::chord::classify::Classify;
use crate::chord::extension::Extension;
use crate::chord::triad::Triad;
use crate::interval::{GetIntervals, Interval};
use crate::note::Note;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: Note,
    pub intervals: Vec<Interval>,
}

impl Chord {
    pub fn triad(root: Note, triad: Triad) -> Self {
        Self {
            root,
            intervals: triad.intervals(),
        }
    }

    pub fn build(root: Note, triad: Triad, extension: Option<Extension>) -> Self {
        let mut intervals = triad.intervals();
        if let Some(extension) = extension {
            intervals.append(&mut extension.intervals());
        }

        Self { root, intervals }
    }

    pub fn from_intervals(root: Note, intervals: Vec<Interval>) -> Self {
        Self { root, intervals }
    }

    pub fn notes(&self) -> Vec<Note> {
        let mut notes = Vec::new();

        for interval in &self.intervals {
            notes.push(self.root.transposed(interval));
        }

        return notes;
    }

    pub fn notate(&self) -> Option<String> {
        if let Some(triad) = Triad::classify(self) {
            if let Some(extension) = Extension::classify(self) {
                if let Some(extended) = triad.extend(&extension) {
                    Some(format!("{}{}", self.root, extended))
                } else {
                    None
                }
            } else {
                Some(format!("{}{}", self.root, triad))
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(notated) = self.notate() {
            write!(f, "{}", notated)
        } else {
            write!(f, "{}", "Unknown Chord")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_notes_in_chords() {
        let cmaj = Chord::triad(Note::C, Triad::Maj);
        assert_eq!(cmaj.notes(), vec![Note::C, Note::E, Note::G]);

        let cmin = Chord::triad(Note::C, Triad::Min);
        assert_eq!(cmin.notes(), vec![Note::C, Note::DSharp, Note::G]);

        let amaj = Chord::triad(Note::A, Triad::Maj);
        assert_eq!(amaj.notes(), vec![Note::A, Note::CSharp, Note::E]);

        let fdim = Chord::triad(Note::F, Triad::Dim);
        assert_eq!(fdim.notes(), vec![Note::F, Note::GSharp, Note::B]);

        let e7 = Chord::build(Note::E, Triad::Maj, Some(Extension::MinorSeventh));
        assert_eq!(e7.notes(), vec![Note::E, Note::GSharp, Note::B, Note::D]);

        let gmaj7 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorSeventh));
        assert_eq!(gmaj7.notes(), vec![Note::G, Note::B, Note::D, Note::FSharp]);

        let bmin7 = Chord::build(Note::B, Triad::Min, Some(Extension::MinorSeventh));
        assert_eq!(bmin7.notes(), vec![Note::B, Note::D, Note::FSharp, Note::A]);
    }

    #[test]
    fn notating_triad_chords() {
        let cmaj = Chord::triad(Note::C, Triad::Maj);
        assert_eq!(cmaj.notate(), Some("Cmaj".to_string()));

        let cmin = Chord::triad(Note::C, Triad::Min);
        assert_eq!(cmin.notate(), Some("Cmin".to_string()));

        let amaj = Chord::triad(Note::A, Triad::Maj);
        assert_eq!(amaj.notate(), Some("Amaj".to_string()));

        let fdim = Chord::triad(Note::F, Triad::Dim);
        assert_eq!(fdim.notate(), Some("Fdim".to_string()));
    }

    #[test]
    fn notating_chords_with_extensions() {
        let e7 = Chord::build(Note::E, Triad::Maj, Some(Extension::MinorSeventh));
        assert_eq!(e7.notate(), Some("E7".to_string()));

        let gmaj7 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorSeventh));
        assert_eq!(gmaj7.notate(), Some("Gmaj7".to_string()));

        let bmin7 = Chord::build(Note::B, Triad::Min, Some(Extension::MinorSeventh));
        assert_eq!(bmin7.notate(), Some("Bmin7".to_string()));
    }

    #[test]
    fn fmt_display_chords() {
        let cmaj = Chord::triad(Note::C, Triad::Maj);
        assert_eq!(cmaj.to_string(), cmaj.notate().unwrap());

        let gmaj7 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorSeventh));
        assert_eq!(gmaj7.to_string(), gmaj7.notate().unwrap());
    }
}
