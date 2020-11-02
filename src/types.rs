use derive_more::{Add, FromStr};

pub type Number = i32;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State(pub String);
impl State {
    pub fn value<'a>(&'a self) -> &'a str {
        &self.0
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
}

#[derive(Debug)]
pub enum HeadMoveDirection {
    Left,
    Right,
    Stay,
}

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;
