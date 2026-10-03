use crate::chord::Extension;
use crate::chord::classify::Classify;
use crate::interval::{GetIntervals, Interval};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum Triad {
    Maj,
    Min,
    Dim,
}
impl Classify<Triad> for Triad {}

impl Triad {
    pub fn extend(&self, extension: &Extension) -> Option<String> {
        match self {
            Triad::Maj => match extension {
                Extension::MajorSeventh => Some("maj7".to_string()),
                Extension::MinorSeventh => Some("7".to_string()),
            },
            Triad::Min => match extension {
                Extension::MajorSeventh => None,
                Extension::MinorSeventh => Some("min7".to_string()),
            },
            Triad::Dim => None,
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
