mod chord;
mod interval;
mod key;
mod note;

use chord::*;
use key::*;

pub fn all_keys() -> Vec<Key> {
    let mut key_list: Vec<Key> = Vec::new();
    for note in enum_iterator::all::<Note>() {
        for mode in enum_iterator::all::<Mode>() {
            key_list.push(Key::new(note.clone(), mode));
        }
    }

    return key_list;
}

fn key_contains_chord(key: &Key, chord: &Chord) -> bool {
    key.chords().iter().find(|(_, c)| c == chord).is_some()
}
pub fn find_keys_containing_chord(chord: Chord) -> Vec<Key> {
    return all_keys()
        .into_iter()
        .filter(|key| key_contains_chord(&key, &chord))
        .collect();
}

fn key_contains_chords(key: &Key, chords: &Vec<Chord>) -> bool {
    chords
        .iter()
        .all(|chord| key.chords().iter().find(|(_, c)| c == chord).is_some())
}
pub fn find_keys_containing_chords(chords: Vec<Chord>) -> Vec<Key> {
    return all_keys()
        .into_iter()
        .filter(|key| key_contains_chords(&key, &chords))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_keys_containing_single_chord() {
        let keys_with_cmaj = find_keys_containing_chord(Chord {
            root: Note::C,
            chord_type: ChordType::Maj,
        });
        assert_eq!(keys_with_cmaj.len(), 6);
        assert!(keys_with_cmaj.contains(&Key::of(Note::C, Mode::Major)));
        assert!(keys_with_cmaj.contains(&Key::of(Note::F, Mode::Major)));
        assert!(keys_with_cmaj.contains(&Key::of(Note::G, Mode::Major)));
        assert!(keys_with_cmaj.contains(&Key::of(Note::A, Mode::Minor)));
        assert!(keys_with_cmaj.contains(&Key::of(Note::D, Mode::Minor)));
        assert!(keys_with_cmaj.contains(&Key::of(Note::E, Mode::Minor)));

        let keys_with_amin = find_keys_containing_chord(Chord {
            root: Note::A,
            chord_type: ChordType::Min,
        });
        assert_eq!(keys_with_amin.len(), 6);
        assert!(keys_with_amin.contains(&Key::of(Note::A, Mode::Minor)));
        assert!(keys_with_amin.contains(&Key::of(Note::D, Mode::Minor)));
        assert!(keys_with_amin.contains(&Key::of(Note::E, Mode::Minor)));
        assert!(keys_with_amin.contains(&Key::of(Note::C, Mode::Major)));
        assert!(keys_with_amin.contains(&Key::of(Note::F, Mode::Major)));
        assert!(keys_with_amin.contains(&Key::of(Note::G, Mode::Major)));

        let keys_with_fdim = find_keys_containing_chord(Chord {
            root: Note::F,
            chord_type: ChordType::Dim,
        });
        assert_eq!(keys_with_fdim.len(), 2);
        assert!(keys_with_fdim.contains(&Key::of(Note::DSharp, Mode::Minor)));
        assert!(keys_with_fdim.contains(&Key::of(Note::FSharp, Mode::Major)));
    }

    #[test]
    fn find_keys_containing_multiple_chords() {
        let keys_with_cmaj_and_emin = find_keys_containing_chords(vec![
            Chord {
                root: Note::C,
                chord_type: ChordType::Maj,
            },
            Chord {
                root: Note::E,
                chord_type: ChordType::Min,
            },
        ]);
        assert_eq!(keys_with_cmaj_and_emin.len(), 4);
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(Note::C, Mode::Major)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(Note::G, Mode::Major)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(Note::A, Mode::Minor)));
        assert!(keys_with_cmaj_and_emin.contains(&Key::of(Note::E, Mode::Minor)));

        let keys_with_cmaj_and_cmin = find_keys_containing_chords(vec![
            Chord {
                root: Note::C,
                chord_type: ChordType::Maj,
            },
            Chord {
                root: Note::C,
                chord_type: ChordType::Min,
            },
        ]);
        assert_eq!(keys_with_cmaj_and_cmin.len(), 0);

        let keys_with_emaj_and_dsharpdim = find_keys_containing_chords(vec![
            Chord {
                root: Note::E,
                chord_type: ChordType::Maj,
            },
            Chord {
                root: Note::DSharp,
                chord_type: ChordType::Dim,
            },
        ]);
        assert_eq!(keys_with_emaj_and_dsharpdim.len(), 2);
        assert!(keys_with_emaj_and_dsharpdim.contains(&Key::of(Note::E, Mode::Major)));
        assert!(keys_with_emaj_and_dsharpdim.contains(&Key::of(Note::CSharp, Mode::Minor)));

        let keys_with_cmaj_and_dmin_and_emin = find_keys_containing_chords(vec![
            Chord {
                root: Note::C,
                chord_type: ChordType::Maj,
            },
            Chord {
                root: Note::D,
                chord_type: ChordType::Min,
            },
            Chord {
                root: Note::E,
                chord_type: ChordType::Min,
            },
        ]);
        assert_eq!(keys_with_cmaj_and_dmin_and_emin.len(), 2);
        assert!(keys_with_cmaj_and_dmin_and_emin.contains(&Key::of(Note::C, Mode::Major)));
        assert!(keys_with_cmaj_and_dmin_and_emin.contains(&Key::of(Note::A, Mode::Minor)));
    }

    #[test]
    fn find_keys_containing_c_major_and_d_minor_and_e_minor() {
        let keys = find_keys_containing_chords(vec![
            Chord {
                root: Note::C,
                chord_type: ChordType::Maj,
            },
            Chord {
                root: Note::D,
                chord_type: ChordType::Min,
            },
            Chord {
                root: Note::E,
                chord_type: ChordType::Min,
            },
        ]);

        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&Key::of(Note::C, Mode::Major)));
        assert!(keys.contains(&Key::of(Note::A, Mode::Minor)));
    }
}
