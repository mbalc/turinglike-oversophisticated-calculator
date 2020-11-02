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
    pub fn apply_transitions(mut self, transitions: &Vec<Transition>) -> Run {
        let mut new_configs = HashSet::new();
        for trans in transitions.iter() {
            for cfg in self.current_configs.iter() {
                if trans.applicable_to(cfg) {
                    new_configs.insert(cfg.apply(trans));
                }
            }
        } // TODO drop new_configs that have already been visited earlier

        self.visited_configs.extend(new_configs.iter().cloned());
        Run {
            visited_configs: self.visited_configs,
            current_configs: new_configs,
            current_step_no: self.current_step_no + 1,
        }
    }
    pub fn is_accepting_run_reached(&self) -> bool {
        self.visited_configs
            .iter()
            .find(|cfg| cfg.state.value() == StdStates::accept)
            .is_some()
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Config {
    pub tape: Tape,
    pub state: State,
}
impl Config {
    pub fn apply(&self, trans: &Transition) -> Config {
        // TODO
        self.clone()
    }
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
        let mut step_configs = HashSet::new();
        step_configs.insert(Config {
            tape: Tape::new(input_word),
            state: State(StdStates::start.to_string()),
        });

        let initial_run_data = Run {
            visited_configs: step_configs.clone(),
            current_configs: step_configs,
            current_step_no: 0,
        };

        Ok(ClassicMachine {
            run: initial_run_data,
            transitions: loader::transitions_from_file(file_path)?,
            execution_limit,
        })
    }

    fn time_limit_reached(&self) -> bool {
        self.run.current_step_no >= self.execution_limit
    }

    pub fn run_with_limit(mut self) -> () {
        while !self.time_limit_reached() && !self.run.is_accepting_run_reached() {
            self.run = self.run.apply_transitions(&self.transitions);
        }
        if self.run.is_accepting_run_reached() {
            println!("OK")
        } else {
            println!("FAIL")
        }
    }
}
