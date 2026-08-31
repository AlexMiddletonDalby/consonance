pub use crate::{
    chord::{Chord, ChordType},
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

pub struct ChordFormula {
    interval: Interval,
    chord_type: ChordType,
}
impl ChordFormula {
    pub const fn new(interval: Interval, chord_type: ChordType) -> Self {
        Self {
            interval,
            chord_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, enum_iterator::Sequence)]
pub enum Mode {
    Major,
    Minor,
}
impl Mode {
    fn chord_formulae(&self) -> Vec<ChordFormula> {
        match self {
            Mode::Major => vec![
                ChordFormula::new(Interval::Root, ChordType::Maj),
                ChordFormula::new(Interval::MajorSecond, ChordType::Min),
                ChordFormula::new(Interval::MajorThird, ChordType::Min),
                ChordFormula::new(Interval::Fourth, ChordType::Maj),
                ChordFormula::new(Interval::Fifth, ChordType::Maj),
                ChordFormula::new(Interval::MajorSixth, ChordType::Min),
                ChordFormula::new(Interval::MajorSeventh, ChordType::Dim),
            ],
            Mode::Minor => vec![
                ChordFormula::new(Interval::Root, ChordType::Min),
                ChordFormula::new(Interval::MajorSecond, ChordType::Dim),
                ChordFormula::new(Interval::MinorThird, ChordType::Maj),
                ChordFormula::new(Interval::Fourth, ChordType::Min),
                ChordFormula::new(Interval::Fifth, ChordType::Min),
                ChordFormula::new(Interval::MinorSixth, ChordType::Maj),
                ChordFormula::new(Interval::MinorSeventh, ChordType::Maj),
            ],
        }
    }
}
impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
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
        for formula in self.mode.chord_formulae() {
            chords.push((
                degree.clone(),
                Chord {
                    root: self.root.transposed(formula.interval),
                    chord_type: formula.chord_type,
                },
            ));
            if let Some(next_degree) = enum_iterator::next(&degree) {
                degree = next_degree
            } else {
                return chords;
            }
        }

        return chords;
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
                (
                    Degree::I,
                    Chord {
                        root: Note::C,
                        chord_type: ChordType::Maj,
                    }
                ),
                (
                    Degree::II,
                    Chord {
                        root: Note::D,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::III,
                    Chord {
                        root: Note::E,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::IV,
                    Chord {
                        root: Note::F,
                        chord_type: ChordType::Maj,
                    }
                ),
                (
                    Degree::V,
                    Chord {
                        root: Note::G,
                        chord_type: ChordType::Maj,
                    }
                ),
                (
                    Degree::VI,
                    Chord {
                        root: Note::A,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::VII,
                    Chord {
                        root: Note::B,
                        chord_type: ChordType::Dim,
                    }
                )
            ]
        );

        assert_eq!(
            Key::of(Note::A, Mode::Minor).chords(),
            vec![
                (
                    Degree::I,
                    Chord {
                        root: Note::A,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::II,
                    Chord {
                        root: Note::B,
                        chord_type: ChordType::Dim,
                    }
                ),
                (
                    Degree::III,
                    Chord {
                        root: Note::C,
                        chord_type: ChordType::Maj,
                    }
                ),
                (
                    Degree::IV,
                    Chord {
                        root: Note::D,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::V,
                    Chord {
                        root: Note::E,
                        chord_type: ChordType::Min,
                    }
                ),
                (
                    Degree::VI,
                    Chord {
                        root: Note::F,
                        chord_type: ChordType::Maj,
                    }
                ),
                (
                    Degree::VII,
                    Chord {
                        root: Note::G,
                        chord_type: ChordType::Maj,
                    }
                )
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
