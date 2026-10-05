use crate::interval::{GetIntervals, Interval};
use std::fmt;

#[derive(Clone, Copy, Debug, enum_iterator::Sequence)]
pub enum Extension {
    MajorSeventh,
    MinorSeventh,
}
impl GetIntervals for Extension {
    fn intervals(&self) -> Vec<Interval> {
        match self {
            Extension::MajorSeventh => {
                vec![Interval::MajorSeventh]
            }
            Extension::MinorSeventh => {
                vec![Interval::MinorSeventh]
            }
        }
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MajorSeventh => "maj7",
                Self::MinorSeventh => "min7",
            }
        )
    }
}
