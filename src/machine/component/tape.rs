use crate::types::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tape {
    content: Vec<TapeEntry>,
    head_idx: usize,
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

    pub fn write_to_head(&mut self, value: &TapeEntry) {
        self.content[self.head_idx] = value.clone()
    }

    pub fn read_from_head(&self) -> &TapeEntry {
        &self.content[self.head_idx]
    }

    fn move_left(&mut self) {
        if self.content[self.head_idx] == TapeEntry::BLANK {
            self.content.pop();
        }
        self.head_idx = std::cmp::max(self.head_idx - 1, 0)
    }

    fn move_right(&mut self) {
        self.head_idx += 1;
        assert!(self.head_idx <= self.content.len());
        if self.head_idx == self.content.len() {
            self.content.push(TapeEntry::BLANK);
        }
    }

    pub fn move_head(&mut self, direction: &HeadMoveDirection) {
        match direction {
            HeadMoveDirection::Left => self.move_left(),
            HeadMoveDirection::Right => self.move_right(),
            HeadMoveDirection::Stay => (),
        }
    }
}
