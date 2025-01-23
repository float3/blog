use super::chordbase::ChordBase;
use crate::{
    defaults::IntegerType,
    note::note::Note,
    pitch::pitch::{simplify_multiple_enharmonics, Pitch},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub(crate) struct Chord {
    chord_base: ChordBase,
    notes: RefCell<Vec<Rc<Note>>>,
    common_name: String,
    pub(crate) pitched_common_name: String,
}

impl Chord {
    pub(crate) fn new(notes: &str) -> Result<Chord, &'static str> {
        println!("chord.new()");
        // if let Some(notes) = &notes {
        //     if notes.iter().any(|n| {
        //         matches!(n.note_type, NoteType::General(_))
        //             && !matches!(n.note_type, NoteType::Specific(_))
        //             && !matches!(n.note_type, NoteType::Chord(_))
        //     }) {
        //         return Err("Use a PercussionChord to contain Unpitched objects");
        //     }
        // }

        // let mut chord = Chord {
        //     _notes: notes.clone().unwrap_or_else(Vec::new),
        //     // initialize other fields here...
        // };

        // if let Some(notes) = &notes {
        //     if notes
        //         .iter()
        //         .all(|n| matches!(n.note_type, NoteType::Specific(_)))
        //     {
        //         chord.simplify_enharmonics_in_place();
        //     }
        // }

        // Ok(chord)
        let mut chord = Chord {
            notes: RefCell::new(
                notes
                    .split(' ')
                    .map(|note| Rc::new(Note::new(note.to_string())))
                    .collect(),
            ),

            common_name: String::new(),
            pitched_common_name: String::new(),
            chord_base: ChordBase::new(),
        };

        chord.common_name = chord.common_name();
        chord.pitched_common_name = chord.pitched_common_name();

        Ok(chord)
    }

    pub(crate) fn pitches(&self) -> Vec<Pitch> {
        self.notes
            .borrow()
            .iter()
            .map(|note| note.pitch.clone())
            .collect()
    }

    fn common_name(&self) -> String {
        todo!()
    }

    fn pitched_common_name(&self) -> String {
        let name_str = self.common_name();
        if name_str == "empty chord" {
            return name_str;
        }
        if name_str == "note" || name_str == "unison" {
            return self.pitches()[0].name.clone();
        }

        if self.pitch_class_cardinality() <= 2
            || name_str.contains("enharmonic")
            || name_str.contains("forte class")
            || name_str.contains("semitone")
        {
            let bass = self.bass();
            let bass_name = bass.name.replace('-', "b");
            format!("{} above {}", name_str, bass_name)
        } else {
            let root = self.root().unwrap_or_else(|| self.pitches()[0].clone());
            let root_name = root.name.replace('-', "b");
            format!("{}-{}", root_name, name_str)
        }
    }

    fn pitch_class_cardinality(&self) -> usize {
        self.unordered_pitch_classes().len()
    }

    fn simplify_enharmonics_in_place(&mut self) {
        let pitches = simplify_multiple_enharmonics(self.pitches());
        for (i, pitch) in pitches.iter().enumerate() {
            if let Some(note) = Rc::get_mut(&mut self.notes.borrow_mut()[i]) {
                note.pitch = pitch.clone();
            }
        }
    }

    fn unordered_pitch_classes(&self) -> Vec<IntegerType> {
        let mut vec = vec![];
        for p in self.pitches() {
            vec.push(p.pitch_class());
        }
        vec
    }

    fn bass(&self) -> Pitch {
        todo!()
    }

    fn root(&self) -> Option<Pitch> {
        todo!()
    }
}
