use crate::machine::classic::Config;
use crate::types::*;

#[derive(Debug)]
pub struct Transition {
    pub state_before: State,
    pub state_after: State,
    pub tape_value_before: TapeEntry,
    pub tape_value_after: TapeEntry,
    pub tape_head_move_direction: HeadMoveDirection,
}

fn tape_head_move_from_char(symbol: char) -> AppResult<HeadMoveDirection> {
    match symbol {
        'L' => Ok(HeadMoveDirection::Left),
        'R' => Ok(HeadMoveDirection::Right),
        'S' => Ok(HeadMoveDirection::Stay),
        _ => Err(format!("bad tape head move description symbol {}", symbol).into()),
    }
}

impl Transition {
    pub fn from_description(description: &str) -> AppResult<Transition> {
        let (state1, val1, state2, val2, dir) = scan_fmt!(
            description,
            "{} {} {} {} {}",
            String,
            Number,
            String,
            Number,
            char
        )?;

        let head_move_direction = tape_head_move_from_char(dir)?;

        Ok(Transition {
            state_before: State(state1),
            state_after: State(state2),
            tape_value_before: TapeEntry(val1),
            tape_value_after: TapeEntry(val2),
            tape_head_move_direction: head_move_direction,
        })
    }

    pub fn applicable_to(&self, cfg: &Config) -> bool {
        self.state_before == cfg.state && &self.tape_value_before == cfg.tape.read_from_head()
    }
}
