use crate::interval::GetIntervals;
use crate::{Chord, Extension, Interval, Triad};
use enum_iterator::Sequence;

trait Classify<S: Sequence + GetIntervals> {
    fn classify(chord: &Chord) -> Option<S> {
        enum_iterator::all::<S>()
            .filter(|s| s.intervals().iter().all(|i| chord.intervals.contains(i)))
            .max_by(|a, b| a.intervals().len().cmp(&b.intervals().len()))
    }
}
impl Classify<Triad> for Triad {}
impl Classify<Extension> for Extension {}

struct Progress(Vec<(Interval, bool)>);
impl Progress {
    fn new(chord: &Chord) -> Self {
        let intervals = chord.intervals.clone();
        Self(intervals.iter().map(|i| (i.clone(), false)).collect())
    }

    fn mark<I: GetIntervals>(&mut self, i: &I) {
        let intervals = i.intervals();
        for (interval, state) in &mut self.0 {
            if intervals.contains(interval) {
                *state = true;
            }
        }
    }

    fn completely_classified(&self) -> bool {
        self.0.iter().all(|(_, classified)| *classified)
    }
}

pub struct Details {
    pub triad: Triad,
    pub extension: Option<Extension>,
}

pub fn classify(chord: &Chord) -> Result<Details, &'static str> {
    let mut progress = Progress::new(chord);
    if let Some(triad) = Triad::classify(chord) {
        progress.mark(&triad);

        if progress.completely_classified() {
            return Ok(Details {
                triad,
                extension: None,
            });
        }

        if let Some(extension) = Extension::classify(chord) {
            progress.mark(&extension);

            if progress.completely_classified() {
                return Ok(Details {
                    triad,
                    extension: Some(extension),
                });
            }
        }
    }

    Err("Could not classify chord")
}
