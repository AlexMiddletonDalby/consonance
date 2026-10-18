#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Octave(i8);
impl Octave {
    pub const MIDDLE_C: i8 = 4;

    pub fn build(midi_number: i8) -> Self {
        Octave(midi_number)
    }

    pub fn middle_c() -> Self {
        Self(Self::MIDDLE_C)
    }

    pub fn plus(self, val: i8) -> Self {
        Self(self.0 + val)
    }

    pub fn minus(self, val: i8) -> Self {
        Self(self.0 - val)
    }

    pub fn relative_to_middle_c(&self) -> i8 {
        self.0 - Self::MIDDLE_C
    }

    pub fn midi_number(&self) -> i8 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn building_octaves_from_midi_number() {
        let octave = Octave::build(4);
        assert_eq!(octave.0, 4);
    }

    #[test]
    fn building_octaves_relative_to_middle_c() {
        let middle_c = Octave::middle_c();
        assert_eq!(middle_c.0, Octave::MIDDLE_C);

        let above = Octave::middle_c().plus(2);
        assert_eq!(above.0, Octave::MIDDLE_C + 2);

        let below = Octave::middle_c().minus(3);
        assert_eq!(below.0, Octave::MIDDLE_C - 3);
    }

    #[test]
    fn getting_octave_value_relative_to_middle_c() {
        let middle_c = Octave::middle_c();
        assert_eq!(middle_c.relative_to_middle_c(), 0);

        let above = Octave::middle_c().plus(2);
        assert_eq!(above.relative_to_middle_c(), 2);

        let below = Octave::middle_c().minus(3);
        assert_eq!(below.relative_to_middle_c(), -3);
    }

    #[test]
    fn getting_octave_midi_number() {
        let middle_c = Octave::middle_c();
        assert_eq!(middle_c.midi_number(), 4);

        let above = Octave::middle_c().plus(2);
        assert_eq!(above.midi_number(), 6);

        let below = Octave::middle_c().minus(3);
        assert_eq!(below.midi_number(), 1);
    }
}
