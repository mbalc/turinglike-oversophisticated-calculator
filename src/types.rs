use derive_more::{Add, FromStr};

#[derive(Debug, Hash, PartialOrd, FromStr, PartialEq, Eq, Add)]
pub struct Number(pub i32);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct State(pub String);
impl State {
    pub fn value<'a>(&'a self) -> &'a str {
        &self.0
    }
}

pub mod StdStates {
    pub const start: &str = "start";
    pub const accept: &str = "accept";
    pub const reject: &str = "reject";
}

#[derive(Debug, Hash, PartialOrd, FromStr, PartialEq, Eq, Add)]
pub struct TapeEntry(pub i32);
impl TapeEntry {
    pub const blank: TapeEntry = TapeEntry(0);
}

#[derive(Debug)]
pub enum TapeHeadMove {
    Left,
    Right,
    Stay,
}

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;
