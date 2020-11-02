use core::ops::{Bound, Range, RangeBounds};
use derive_more::{Add, FromStr};
use intbits::{Bits, BitsIndex};

pub type Number = u64;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State(pub String);
impl State {
    pub fn value<'a>(&'a self) -> &'a str {
        &self.0
    }
    pub fn to_string(&self) -> String {
        self.value().clone().to_string()
    }
}

pub mod std_states {
    pub const START: &str = "start";
    pub const ACCEPT: &str = "accept";

    /* REJECT unused because if we got a valid machine, this state wouldn't be used as
    a prerequisite transition state anyway - so here I'm actually allowing
    transitions >from< `reject` state */
    // pub const REJECT: &str = "reject";
}

#[derive(Debug, Hash, PartialOrd, FromStr, PartialEq, Eq, Add, Clone)]
pub struct TapeEntry(pub Number);
impl TapeEntry {
    pub const BLANK: TapeEntry = TapeEntry(0);
    pub const MAX_BIT_COUNT: Number = 30;

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    // forwarding Bits trait manually
    // disregard the following - I'm only forwarding methods from TapeEntry to Number
    // because the dev of the `intbits` crate seemingly forgot to export derive trait macro
    fn bit(&self, i: Number) -> bool {
        self.0.bit(i)
    }
    fn bits(&self, range: std::ops::Range<Number>) -> Number {
        self.0.bits(range)
    }
    fn set_bit(&mut self, i: Number, bit: bool) {
        self.0.set_bit(i, bit)
    }
    fn set_bits(&mut self, range: Range<Number>, bits: Number) {
        self.0.set_bits(range, bits)
    }
    fn with_bit(&mut self, i: Number, bit: bool) -> TapeEntry {
        TapeEntry(self.0.with_bit(i, bit))
    }
    fn with_bits(&mut self, range: Range<Number>, bits: Number) -> TapeEntry {
        TapeEntry(self.0.with_bits(range, bits))
    }
}

#[derive(Debug, Clone)]
pub enum HeadMoveDirection {
    Left,
    Right,
    Stay,
}

pub fn direction_to_string(direction: &HeadMoveDirection) -> String {
    match direction {
        Left => "L".to_string(),
        Right => "R".to_string(),
        Stay => "S".to_string(),
    }
}

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Clone)]
pub struct SuperTapeEntry {
    pub first_letter: TapeEntry,
    pub second_letter: TapeEntry,
    pub has_first_tape_head: bool,
    pub has_second_tape_head: bool,
    pub is_start: bool,
}
impl SuperTapeEntry {
    const TAPE_ENTRY_DATA_RANGE: Range<Number> = 0..TapeEntry::MAX_BIT_COUNT;
    const FIRST_LETTER_RANGE: Range<Number> = 0..TapeEntry::MAX_BIT_COUNT;
    const SECOND_LETTER_RANGE: Range<Number> =
        TapeEntry::MAX_BIT_COUNT..2 * TapeEntry::MAX_BIT_COUNT;
    const HAS_FIRST_TAPE_HEAD_ID: Number = 2 * TapeEntry::MAX_BIT_COUNT;
    const HAS_SECOND_TAPE_HEAD_ID: Number = 2 * TapeEntry::MAX_BIT_COUNT + 1;
    const IS_START_ID: Number = 2 * TapeEntry::MAX_BIT_COUNT + 2;

    pub fn encoded(&self) -> TapeEntry {
        let mut result = TapeEntry(0);
        result
            .with_bits(
                Self::FIRST_LETTER_RANGE,
                self.first_letter.bits(Self::TAPE_ENTRY_DATA_RANGE),
            )
            .with_bits(
                Self::SECOND_LETTER_RANGE,
                self.second_letter.bits(Self::TAPE_ENTRY_DATA_RANGE),
            )
            .with_bit(Self::HAS_FIRST_TAPE_HEAD_ID, self.has_first_tape_head)
            .with_bit(Self::HAS_SECOND_TAPE_HEAD_ID, self.has_second_tape_head)
            .with_bit(Self::IS_START_ID, self.is_start)
    }
    pub fn decode(entry: TapeEntry) -> Self {
        Self {
            first_letter: TapeEntry(entry.bits(Self::FIRST_LETTER_RANGE)),
            second_letter: TapeEntry(entry.bits(Self::SECOND_LETTER_RANGE)),
            has_first_tape_head: entry.bit(Self::HAS_FIRST_TAPE_HEAD_ID),
            has_second_tape_head: entry.bit(Self::HAS_SECOND_TAPE_HEAD_ID),
            is_start: entry.bit(Self::IS_START_ID),
        }
    }
}
