// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
pub type Error = ();

pub struct Scale {
    notes: Vec<Note>,
}

#[derive(Clone, PartialEq)]
pub enum Note {
    Ab,
    A,
    As,
    Bb,
    B,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
}

// chromatic scale, 12 pitches
// - chromatic sharp: A, A#, B, C, C#, D, D#, E, F, F#, G, G#
// - chromatic flat:  A, Bb, B, C, Db, D, Eb, E, F, Gb, G, Ab
// diatonic scale: 7 pitches
// - No Sharps or Flats: C major, a minor
// - Use Sharps: G, D,  A,  E,  B,  F# major, e, b, f#, c#, g#, d# minor
// - Use Flats:  F, Bb, Eb, Ab, Db, Gb major, d, g, c,  f,  bb, eb minor
//
// An interval is the space between two pitches.
// m: interval between two adjacent notes: "half step", or "minor second"
// M: interval between two notes that have an interceding note, a "whole step"
//    or "major second"
//
// The diatonic scales are built using only these two intervals between
// adjacent notes.
// Non-diatonic scales can contain other intervals.
// A: "augmented first" interval, has two interceding notes (e.g.,
// from A to C or Db to E).

impl Scale {
    fn sharp() -> Vec<Note> {
        let sharp = [
            Note::A,
            Note::As,
            Note::B,
            Note::C,
            Note::Cs,
            Note::D,
            Note::Ds,
            Note::E,
            Note::F,
            Note::Fs,
            Note::G,
            Note::Gs,
        ]
        .to_vec();
        return sharp;
    }

    fn flat() -> Vec<Note> {
        let flat = [
            Note::A,
            Note::Bb,
            Note::B,
            Note::C,
            Note::Db,
            Note::D,
            Note::Eb,
            Note::E,
            Note::F,
            Note::Gb,
            Note::G,
            Note::Ab,
        ]
        .to_vec();
        return flat;
    }

    fn is_flat(tonic: &str) -> bool {
        return match tonic {
            "Ab" | "Bb" | "Db" | "Eb" | "F" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => true,
            _ => false,
        };
    }

    fn is_sharp(tonic: &str) -> bool {
        return match tonic {
            "G" | "D" | "A" | "E" | "B" | "Fs" | "e" | "b" | "fs" | "cs" | "gs" | "ds" => true,
            _ => false,
        };
    }
}

impl Scale {
    // Given a tonic, or starting note, and a set of intervals, generate
    // the musical scale starting with the tonic and following the
    // specified interval pattern.
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        // prepare the scale as empty vec of notes
        let mut scale: Vec<Note> = Vec::new();
        // take an iterator on the intervals
        let mut i: std::str::Chars = intervals.chars();

        if Scale::is_flat(tonic) {
        }
        else if Scale::is_sharp(tonic) {
        }
        else {
            return Err(Error::from("nor flat, nor sharp, panic!"));
        }
        return Ok(Scale { notes: scale });
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {}", tonic)
    }

    pub fn enumerate(&self) -> Vec<String> {
        unimplemented!()
    }
}
