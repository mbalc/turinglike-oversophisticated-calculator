use crate::machine::component::tape::Tape;
use crate::machine::component::transition::Transition;
use crate::machine::logic::loader;
use crate::types::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Run {
    visited_configs: HashSet<Config>,
    current_step_no: Number,
    current_configs: HashSet<Config>,
}
impl Run {
    fn is_accepting_run_reached(self) -> bool {
        self.visited_configs
            .iter()
            .find(|cfg| cfg.state.value() == StdStates::accept)
            .is_some()
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct Config {
    tape: Tape,
    state: State,
}

#[derive(Debug)]
pub struct ClassicMachine {
    run: Run,
    transitions: Vec<Transition>,
    execution_limit: Number,
}

impl ClassicMachine {
    pub fn new(
        file_path: String,
        execution_limit: Number,
        input_word: String,
    ) -> AppResult<ClassicMachine> {
        let initial_config = Config {
            tape: Tape::new(input_word),
            state: State(StdStates::start.to_string()),
        };

        let mut step_configs = HashSet::new();
        step_configs.insert(initial_config);

        let initial_run_data = Run {
            visited_configs: HashSet::new(),
            current_configs: step_configs,
            current_step_no: Number(0),
        };

        Ok(ClassicMachine {
            run: initial_run_data,
            transitions: loader::transitions_from_file(file_path)?,
            execution_limit,
        })
    }

    fn time_limit_reached(self) -> bool {
        self.run.current_step_no >= self.execution_limit
    }

    pub fn run_with_limit(self) -> () {}
}
