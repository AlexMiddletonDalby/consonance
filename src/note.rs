pub use crate::interval::Interval;
use std::fmt;

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
    pub fn transposed(&self, interval: Interval) -> Note {
        let mut note = self.clone();
        let semitones = interval.semitones();

        for _ in 0..semitones {
            note = enum_iterator::next_cycle(&note);
        }

        return note;
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
        assert_eq!(Note::C.transposed(Interval::Fifth), Note::G);
        assert_eq!(Note::G.transposed(Interval::Octave), Note::G);
        assert_eq!(Note::E.transposed(Interval::MinorSecond), Note::F);
    }
}
