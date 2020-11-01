pub type Number = i32;

pub type State = String;
pub type TapeEntry = Number;

#[derive(Debug)]
pub enum TapeHeadMove {
    Left,
    Right,
    Stay,
}

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T> = Result<T, AppError>;
