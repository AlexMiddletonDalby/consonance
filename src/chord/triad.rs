use crate::chord::Extension;
use crate::interval::{GetIntervals, Interval};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum Triad {
    Maj,
    Min,
    Dim,
}

impl Triad {
    pub fn extend(&self, extension: &Extension) -> Result<String, &'static str> {
        match self {
            Triad::Maj => match extension {
                Extension::MajorSeventh => Ok("maj7".to_string()),
                Extension::MajorNinth => Ok("maj9".to_string()),
                Extension::MinorSeventh => Ok("7".to_string()),
                Extension::MinorNinth => Ok("9".to_string()),
            },
            Triad::Min => match extension {
                Extension::MajorSeventh => Err("Invalid extension"),
                Extension::MajorNinth => Err("Invalid extension"),
                Extension::MinorSeventh => Ok("min7".to_string()),
                Extension::MinorNinth => Ok("min9".to_string()),
            },
            Triad::Dim => Err("Diminished chords cannot be extended"),
        }
    }
}

impl GetIntervals for Triad {
    fn intervals(&self) -> Vec<Interval> {
        match self {
            Triad::Maj => {
                vec![Interval::Root, Interval::MajorThird, Interval::Fifth]
            }
            Triad::Min => {
                vec![Interval::Root, Interval::MinorThird, Interval::Fifth]
            }
            Triad::Dim => vec![
                Interval::Root,
                Interval::MinorThird,
                Interval::DiminishedFifth,
            ],
        }
    }
}

impl fmt::Display for Triad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Maj => "maj",
                Self::Min => "min",
                Self::Dim => "dim",
            }
        )
    }
}
