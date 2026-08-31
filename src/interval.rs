pub enum Interval {
    Root,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    Fourth,
    DiminishedFifth,
    Fifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}
impl Interval {
    pub fn semitones(&self) -> i8 {
        match self {
            Interval::Root => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::Fourth => 5,
            Interval::DiminishedFifth => 6,
            Interval::Fifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
        }
    }
}
