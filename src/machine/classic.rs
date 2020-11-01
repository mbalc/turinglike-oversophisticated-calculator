use crate::machine::component::tape::Tape;
use crate::machine::component::transition::Transition;
use crate::machine::logic::loader;
use crate::types::*;

#[derive(Debug)]
pub struct ClassicMachine {
    tape: Tape,
    transitions: Vec<Transition>,
}

impl ClassicMachine {
    pub fn from_file(input_word: String, file_path: String) -> AppResult<ClassicMachine> {
        Ok(ClassicMachine {
            tape: Tape::new(input_word),
            transitions: loader::transitions_from_file(file_path)?,
        })
    }

    pub fn run_with_limit(self, execution_limit: Number) -> () {}
}
