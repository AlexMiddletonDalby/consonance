use crate::interval::{GetIntervals, Interval};

#[derive(Clone, Copy, Debug, enum_iterator::Sequence)]
pub enum Extension {
    MajorSeventh,
    MinorSeventh,
    MajorNinth,
    MinorNinth,
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
            Extension::MajorNinth => {
                vec![Interval::MajorSeventh, Interval::MajorNinth]
            }
            Extension::MinorNinth => {
                vec![Interval::MinorSeventh, Interval::MajorNinth]
            }
        }
    }
}
