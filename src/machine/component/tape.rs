use crate::types::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Tape {
    content: Vec<TapeEntry>,
}

impl Tape {
    pub fn new(initial_content: String) -> Tape {
        Tape {
            content: Vec::new(),
        }
    }
}
