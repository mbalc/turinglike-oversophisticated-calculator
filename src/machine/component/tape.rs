use crate::types::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tape {
    content: Vec<TapeEntry>,
    head_idx: Number,
}

impl Tape {
    pub fn new(initial_content: String) -> Tape {
        //dbg!(&initial_content);
        let initial_tape_entries: Vec<TapeEntry> = initial_content
            .chars()
            .map(|ch| {
                let digit = ch.to_digit(10).unwrap(); // TODO catch panic
                TapeEntry(digit as i32)
            })
            .collect();

        Tape {
            content: initial_tape_entries,
            head_idx: 0,
        }
    }
}
