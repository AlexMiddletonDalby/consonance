pub mod chord;
pub mod interval;
pub mod key;
pub mod note;

pub use chord::*;
pub use key::*;

fn key_contains_chords(key: &Key, chords: &Vec<Chord>) -> bool {
    chords
        .iter()
        .all(|chord| key.chords().iter().find(|(_, c)| c == chord).is_some())
}
fn containing_chords(keys: &Vec<Key>, chords: Vec<Chord>) -> Vec<Key> {
    keys.to_owned()
        .into_iter()
        .filter(|key| key_contains_chords(&key, &chords))
        .collect()
}
fn key_contains_degrees(key: &Key, degrees: &Vec<(Degree, Chord)>) -> bool {
    degrees.iter().all(|degree| {
        key.chords()
            .iter()
            .find(|(d, c)| *d == degree.0 && *c == degree.1)
            .is_some()
    })
}
fn containing_degrees(keys: &Vec<Key>, degrees: Vec<(Degree, Chord)>) -> Vec<Key> {
    keys.to_owned()
        .into_iter()
        .filter(|key| key_contains_degrees(&key, &degrees))
        .collect()
}

pub fn all_keys() -> Vec<Key> {
    let mut key_list: Vec<Key> = Vec::new();
    for note in enum_iterator::all::<Note>() {
        for mode in enum_iterator::all::<Mode>() {
            key_list.push(Key::new(note.clone(), mode));
        }
    }

    return key_list;
}

pub fn all_chords(of_type: ChordType) -> Vec<Chord> {
    let mut chord_list: Vec<Chord> = Vec::new();
    for note in enum_iterator::all::<Note>() {
        chord_list.push(Chord::new(note, of_type));
    }

    return chord_list;
}

pub trait Containing<C> {
    fn containing(&self, c: C) -> Self;
}
impl Containing<Vec<Chord>> for Vec<Key> {
    fn containing(&self, chords: Vec<Chord>) -> Self {
        containing_chords(self, chords)
    }
}
impl Containing<Chord> for Vec<Key> {
    fn containing(&self, chord: Chord) -> Self {
        self.containing(vec![chord])
    }
}
impl Containing<Vec<(Degree, Chord)>> for Vec<Key> {
    fn containing(&self, degrees: Vec<(Degree, Chord)>) -> Self {
        containing_degrees(self, degrees)
    }
}
impl Containing<(Degree, Chord)> for Vec<Key> {
    fn containing(&self, degree: (Degree, Chord)) -> Self {
        self.containing(vec![degree])
    }
}

#[cfg(test)]
mod tests {
    use crate::key::ChordType::*;
    use crate::key::Degree::*;
    use crate::key::Mode::*;
    use crate::key::Note::*;

    use super::*;

    #[test]
    fn find_keys_containing_single_chord() {
        let keys_with_cmaj = all_keys().containing(vec![Chord::new(C, Maj)]);
        assert_eq!(keys_with_cmaj.len(), 6);
        assert!(keys_with_cmaj.contains(&Key::of(C, Major)));
        assert!(keys_with_cmaj.contains(&Key::of(F, Major)));
        assert!(keys_with_cmaj.contains(&Key::of(G, Major)));
        assert!(keys_with_cmaj.contains(&Key::of(A, Minor)));
        assert!(keys_with_cmaj.contains(&Key::of(D, Minor)));
        assert!(keys_with_cmaj.contains(&Key::of(E, Minor)));

        let keys_with_amin = all_keys().containing(vec![Chord::new(A, Min)]);
        assert_eq!(keys_with_amin.len(), 6);
        assert!(keys_with_amin.contains(&Key::of(A, Minor)));
        assert!(keys_with_amin.contains(&Key::of(D, Minor)));
        assert!(keys_with_amin.contains(&Key::of(E, Minor)));
        assert!(keys_with_amin.contains(&Key::of(C, Major)));
        assert!(keys_with_amin.contains(&Key::of(F, Major)));
        assert!(keys_with_amin.contains(&Key::of(G, Major)));

        let keys_with_fdim = all_keys().containing(vec![Chord::new(F, Dim)]);
        assert_eq!(keys_with_fdim.len(), 2);
        assert!(keys_with_fdim.contains(&Key::of(DSharp, Minor)));
        assert!(keys_with_fdim.contains(&Key::of(FSharp, Major)));
    }

    #[test]
    fn find_keys_containing_multiple_chords() {
        let keys_with_cmaj_and_emin =
            all_keys().containing(vec![Chord::new(C, Maj), Chord::new(E, Min)]);
        assert_eq!(keys_with_cmaj_and_emin.len(), 4);
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(C, Major)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(G, Major)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(A, Minor)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(E, Minor)));

        let keys_with_cmaj_and_cmin =
            all_keys().containing(vec![Chord::new(C, Maj), Chord::new(C, Min)]);
        assert_eq!(keys_with_cmaj_and_cmin.len(), 0);

        let keys_with_emaj_and_dsharpdim =
            all_keys().containing(vec![Chord::new(E, Maj), Chord::new(DSharp, Dim)]);
        assert_eq!(keys_with_emaj_and_dsharpdim.len(), 2);
        assert!(keys_with_emaj_and_dsharpdim.contains(&Key::of(E, Major)));
        assert!(keys_with_emaj_and_dsharpdim.contains(&Key::of(CSharp, Minor)));

        let keys_with_cmaj_and_dmin_and_emin = all_keys().containing(vec![
            Chord::new(C, Maj),
            Chord::new(D, Min),
            Chord::new(E, Min),
        ]);
        assert_eq!(keys_with_cmaj_and_dmin_and_emin.len(), 2);
        assert!(keys_with_cmaj_and_dmin_and_emin.contains(&Key::of(C, Major)));
        assert!(keys_with_cmaj_and_dmin_and_emin.contains(&Key::of(A, Minor)));
    }

    #[test]
    fn find_keys_containing_single_degree() {
        let keys_with_cmaj_i = all_keys().containing((I, Chord::new(C, Maj)));
        assert_eq!(keys_with_cmaj_i.len(), 1);
        assert!(keys_with_cmaj_i.contains(&Key::of(C, Major)));

        let keys_with_amin_ii = all_keys().containing((II, Chord::new(A, Min)));
        assert_eq!(keys_with_amin_ii.len(), 1);
        assert!(keys_with_amin_ii.contains(&Key::of(G, Major)));

        let keys_with_fdim_vii = all_keys().containing((VII, Chord::new(F, Dim)));
        assert_eq!(keys_with_fdim_vii.len(), 1);
        assert!(keys_with_fdim_vii.contains(&Key::of(FSharp, Major)));
    }

    #[test]
    fn find_keys_containing_multiple_degrees() {
        let keys_with_cmaj_i_and_emin_iii =
            all_keys().containing(vec![(I, Chord::new(C, Maj)), (III, Chord::new(E, Min))]);
        assert_eq!(keys_with_cmaj_i_and_emin_iii.len(), 1);
        assert!(keys_with_cmaj_i_and_emin_iii.contains(&Key::of(C, Major)));

        let keys_with_dsharpdim_ii_and_emaj_iii = all_keys().containing(vec![
            (II, Chord::new(DSharp, Dim)),
            (III, Chord::new(E, Maj)),
        ]);
        assert_eq!(keys_with_dsharpdim_ii_and_emaj_iii.len(), 1);
        assert!(keys_with_dsharpdim_ii_and_emaj_iii.contains(&Key::of(CSharp, Minor)));

        let keys_with_cmaj_i_and_emin_iii_and_fmaj_iv = all_keys().containing(vec![
            (I, Chord::new(C, Maj)),
            (III, Chord::new(E, Min)),
            (IV, Chord::new(G, Maj)),
        ]);
        assert_eq!(keys_with_cmaj_i_and_emin_iii_and_fmaj_iv.len(), 0);
    }

    #[test]
    fn find_keys_containing_chords_and_degrees() {
        let keys_with_cmaj_i_and_a_min = all_keys()
            .containing((I, Chord::new(C, Maj)))
            .containing(Chord::new(A, Min));
        assert_eq!(keys_with_cmaj_i_and_a_min.len(), 1);
        assert!(keys_with_cmaj_i_and_a_min.contains(&Key::of(C, Mode::Major)));
    }
}
