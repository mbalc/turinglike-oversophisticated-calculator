use crate::machine::classic::Config;
use crate::types::*;

#[derive(Debug)]
pub struct Transition {
    from_state: State,
    to_state: State,
    from_tape_value: TapeEntry,
    to_tape_value: TapeEntry,
    tape_head_move: TapeHeadMove,
}

fn tape_head_move_from_char(symbol: char) -> AppResult<TapeHeadMove> {
    match symbol {
        'L' => Ok(TapeHeadMove::Left),
        'R' => Ok(TapeHeadMove::Right),
        'S' => Ok(TapeHeadMove::Stay),
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
            from_state: State(state1),
            to_state: State(state2),
            from_tape_value: TapeEntry(val1),
            to_tape_value: TapeEntry(val2),
            tape_head_move: head_move_direction,
        })
    }

    pub fn applicable_to(&self, cfg: &Config) -> bool {
        self.from_state == cfg.state // TODO also compare tape head values
    }
}
