use crate::machine::classic::Config;
use crate::machine::component::transition as one_tape_transition;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct DoubleTransition {
    pub state_before: State,
    pub state_after: State,
    pub first_tape_value_before: TapeEntry,
    pub first_tape_value_after: TapeEntry,
    pub first_tape_head_move_direction: HeadMoveDirection,
    pub second_tape_value_before: TapeEntry,
    pub second_tape_value_after: TapeEntry,
    pub second_tape_head_move_direction: HeadMoveDirection,
}

impl DoubleTransition {
    pub fn from_description(description: &str) -> AppResult<Self> {
        let (state1, val11, val12, state2, val21, val22, dir1, dir2) = scan_fmt!(
            description,
            "{} {} {} {} {} {} {} {}",
            String,
            Number,
            Number,
            String,
            Number,
            Number,
            char,
            char
        )?;

        Ok(Self {
            state_before: State(state1),
            state_after: State(state2),
            first_tape_value_before: TapeEntry(val11),
            first_tape_value_after: TapeEntry(val21),
            first_tape_head_move_direction: one_tape_transition::tape_head_move_from_char(dir1)?,
            second_tape_value_before: TapeEntry(val12),
            second_tape_value_after: TapeEntry(val22),
            second_tape_head_move_direction: one_tape_transition::tape_head_move_from_char(dir2)?,
        })
    }
}
