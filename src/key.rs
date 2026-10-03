pub use crate::{
    chord::{Chord, Triad},
    interval::Interval,
    note::Note,
};

use std::fmt;

#[derive(Clone, Debug, PartialEq, enum_iterator::Sequence)]
pub enum Degree {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}
impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TriadFormula {
    interval: Interval,
    triad: Triad,
}
impl TriadFormula {
    pub const fn new(interval: Interval, triad: Triad) -> Self {
        Self { interval, triad }
    }
}
#[derive(Clone, Debug, PartialEq, enum_iterator::Sequence)]
pub enum Mode {
    Major,
    Minor,
}
impl Mode {
    fn formulae(&self) -> Vec<TriadFormula> {
        match self {
            Mode::Major => vec![
                TriadFormula::new(Interval::Root, Triad::Maj),
                TriadFormula::new(Interval::MajorSecond, Triad::Min),
                TriadFormula::new(Interval::MajorThird, Triad::Min),
                TriadFormula::new(Interval::Fourth, Triad::Maj),
                TriadFormula::new(Interval::Fifth, Triad::Maj),
                TriadFormula::new(Interval::MajorSixth, Triad::Min),
                TriadFormula::new(Interval::MajorSeventh, Triad::Dim),
            ],
            Mode::Minor => vec![
                TriadFormula::new(Interval::Root, Triad::Min),
                TriadFormula::new(Interval::MajorSecond, Triad::Dim),
                TriadFormula::new(Interval::MinorThird, Triad::Maj),
                TriadFormula::new(Interval::Fourth, Triad::Min),
                TriadFormula::new(Interval::Fifth, Triad::Min),
                TriadFormula::new(Interval::MinorSixth, Triad::Maj),
                TriadFormula::new(Interval::MinorSeventh, Triad::Maj),
            ],
        }
    }
}
impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    pub root: Note,
    pub mode: Mode,
}
impl Key {
    pub fn new(root: Note, mode: Mode) -> Self {
        Self { root, mode }
    }

    pub fn of(root: Note, mode: Mode) -> Self {
        Key::new(root, mode)
    }

    pub fn chords(&self) -> Vec<(Degree, Chord)> {
        let mut chords = Vec::new();

        let mut degree = Degree::I;
        for formula in self.mode.formulae() {
            chords.push((
                degree.clone(),
                Chord::triad(self.root.transposed(&formula.interval), formula.triad),
            ));
            if let Some(next_degree) = enum_iterator::next(&degree) {
                degree = next_degree
            } else {
                return chords;
            }
        }

        return chords;
    }

    pub fn find_degree(&self, chord: &Chord) -> Option<Degree> {
        let chords = self.chords();
        chords.iter().find_map(|(key_degree, key_chord)| {
            if key_chord == chord {
                return Some(key_degree.clone());
            }

            return None;
        })
    }
}
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.root, self.mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_chords_from_keys() {
        assert_eq!(
            Key::of(Note::C, Mode::Major).chords(),
            vec![
                (Degree::I, Chord::triad(Note::C, Triad::Maj)),
                (Degree::II, Chord::triad(Note::D, Triad::Min)),
                (Degree::III, Chord::triad(Note::E, Triad::Min,)),
                (Degree::IV, Chord::triad(Note::F, Triad::Maj,)),
                (Degree::V, Chord::triad(Note::G, Triad::Maj,)),
                (Degree::VI, Chord::triad(Note::A, Triad::Min,)),
                (Degree::VII, Chord::triad(Note::B, Triad::Dim,))
            ]
        );

        assert_eq!(
            Key::of(Note::A, Mode::Minor).chords(),
            vec![
                (Degree::I, Chord::triad(Note::A, Triad::Min,)),
                (Degree::II, Chord::triad(Note::B, Triad::Dim,)),
                (Degree::III, Chord::triad(Note::C, Triad::Maj,)),
                (Degree::IV, Chord::triad(Note::D, Triad::Min,)),
                (Degree::V, Chord::triad(Note::E, Triad::Min,)),
                (Degree::VI, Chord::triad(Note::F, Triad::Maj,)),
                (Degree::VII, Chord::triad(Note::G, Triad::Maj,))
            ]
        );
    }

    #[test]
    fn display_key_names() {
        assert_eq!(Key::of(Note::C, Mode::Major).to_string(), "C Major");
        assert_eq!(Key::of(Note::A, Mode::Minor).to_string(), "A Minor");
        assert_eq!(Key::of(Note::FSharp, Mode::Minor).to_string(), "F# Minor");
    }
}
