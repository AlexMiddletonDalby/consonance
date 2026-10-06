use crate::chord::extension::Extension;
use crate::chord::triad::Triad;
use crate::classification;
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

    pub fn notate(&self) -> Result<String, &'static str> {
        if let Ok(details) = classification::classify(self) {
            if let Some(extension) = details.extension {
                if let Ok(extended) = details.triad.extend(&extension) {
                    return Ok(format!("{}{}", self.root, extended));
                }
            }

            return Ok(format!("{}{}", self.root, details.triad));
        }

        Err("Unknown chord")
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.notate() {
            Ok(notated) => write!(f, "{}", notated),
            Err(error) => write!(f, "{}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Note::C;

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

        let e9 = Chord::build(Note::E, Triad::Maj, Some(Extension::MinorNinth));
        assert_eq!(
            e9.notes(),
            vec![Note::E, Note::GSharp, Note::B, Note::D, Note::FSharp]
        );

        let gmaj9 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorNinth));
        assert_eq!(
            gmaj9.notes(),
            vec![Note::G, Note::B, Note::D, Note::FSharp, Note::A]
        );

        let bmin9 = Chord::build(Note::B, Triad::Min, Some(Extension::MinorNinth));
        assert_eq!(
            bmin9.notes(),
            vec![Note::B, Note::D, Note::FSharp, Note::A, Note::CSharp]
        );

        let random = Chord::from_intervals(
            Note::D,
            vec![
                Interval::Root,
                Interval::MajorThird,
                Interval::MajorSixth,
                Interval::Octave,
            ],
        );
        assert_eq!(
            random.notes(),
            vec![Note::D, Note::FSharp, Note::B, Note::D]
        );
    }

    #[test]
    fn notating_triad_chords() {
        let cmaj = Chord::triad(Note::C, Triad::Maj);
        assert_eq!(cmaj.notate(), Ok("Cmaj".to_string()));

        let cmin = Chord::triad(Note::C, Triad::Min);
        assert_eq!(cmin.notate(), Ok("Cmin".to_string()));

        let amaj = Chord::triad(Note::A, Triad::Maj);
        assert_eq!(amaj.notate(), Ok("Amaj".to_string()));

        let fdim = Chord::triad(Note::F, Triad::Dim);
        assert_eq!(fdim.notate(), Ok("Fdim".to_string()));
    }

    #[test]
    fn notating_chords_with_extensions() {
        let e7 = Chord::build(Note::E, Triad::Maj, Some(Extension::MinorSeventh));
        assert_eq!(e7.notate(), Ok("E7".to_string()));

        let gmaj7 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorSeventh));
        assert_eq!(gmaj7.notate(), Ok("Gmaj7".to_string()));

        let bmin7 = Chord::build(Note::B, Triad::Min, Some(Extension::MinorSeventh));
        assert_eq!(bmin7.notate(), Ok("Bmin7".to_string()));

        let e9 = Chord::build(Note::E, Triad::Maj, Some(Extension::MinorNinth));
        assert_eq!(e9.notate(), Ok("E9".to_string()));

        let gmaj9 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorNinth));
        assert_eq!(gmaj9.notate(), Ok("Gmaj9".to_string()));

        let bmin9 = Chord::build(Note::B, Triad::Min, Some(Extension::MinorNinth));
        assert_eq!(bmin9.notate(), Ok("Bmin9".to_string()));
    }

    #[test]
    fn notating_chords_from_intervals() {
        let cmaj = Chord::from_intervals(
            C,
            vec![Interval::Root, Interval::MajorThird, Interval::Fifth],
        );
        assert_eq!(cmaj.notate(), Ok("Cmaj".to_string()));

        let gmin7 = Chord::from_intervals(
            Note::G,
            vec![
                Interval::Root,
                Interval::MinorThird,
                Interval::Fifth,
                Interval::MinorSeventh,
            ],
        );
        assert_eq!(gmin7.notate(), Ok("Gmin7".to_string()));

        let no_notes = Chord::from_intervals(Note::A, vec![]);
        assert!(no_notes.notate().is_err());

        let wrong_notes = Chord::from_intervals(
            Note::B,
            vec![Interval::Root, Interval::MinorSecond, Interval::Octave],
        );
        assert!(wrong_notes.notate().is_err());

        let extra_notes = Chord::from_intervals(
            Note::F,
            vec![
                Interval::Root,
                Interval::MajorThird,
                Interval::Fourth,
                Interval::Fifth,
            ],
        );
        assert!(extra_notes.notate().is_err());
    }

    #[test]
    fn fmt_display_chords() {
        let cmaj = Chord::triad(Note::C, Triad::Maj);
        assert_eq!(cmaj.to_string(), cmaj.notate().unwrap());

        let gmaj7 = Chord::build(Note::G, Triad::Maj, Some(Extension::MajorSeventh));
        assert_eq!(gmaj7.to_string(), gmaj7.notate().unwrap());
    }
}
