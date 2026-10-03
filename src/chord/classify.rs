use crate::chord::chord::Chord;
use crate::interval::GetIntervals;
use enum_iterator::Sequence;

pub trait Classify<S: Sequence + GetIntervals> {
    fn classify(chord: &Chord) -> Option<S> {
        enum_iterator::all::<S>()
            .filter(|s| s.intervals().iter().all(|i| chord.intervals.contains(i)))
            .max_by(|a, b| a.intervals().len().cmp(&b.intervals().len()))
    }
}
