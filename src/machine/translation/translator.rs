use crate::machine::component::transition::Transition;
use crate::machine::logic::loader;
use crate::machine::translation::double_transition::DoubleTransition;
use crate::types::*;
use itertools::iproduct;
use std::collections::HashSet;

struct TranslatorInput {
    transitions: Vec<DoubleTransition>,
    states: Vec<State>,
    tape_entries: Vec<TapeEntry>,
}
impl TranslatorInput {
    pub fn new(machine_description: String) -> AppResult<Self> {
        let transitions = loader::two_tape_transitions_from_description(machine_description)?;
        let program_states: HashSet<State> = transitions
            .iter()
            .cloned()
            .map(|trans| vec![trans.state_before, trans.state_after])
            .flatten()
            .collect();

        let mut program_tape_entries: HashSet<TapeEntry> = transitions
            .iter()
            .cloned()
            .map(|trans| {
                vec![
                    trans.first_tape_value_before,
                    trans.first_tape_value_after,
                    trans.second_tape_value_before,
                    trans.second_tape_value_after,
                ]
            })
            .flatten()
            .collect();

        let digits: Vec<TapeEntry> = (0..9).map(|d| TapeEntry(d)).collect();
        program_tape_entries.extend(digits);

        Ok(Self {
            transitions: transitions,
            states: program_states.iter().cloned().collect(),
            tape_entries: program_tape_entries.iter().cloned().collect(),
        })
    }
}

pub struct MachineTranslator {
    input: TranslatorInput,
    possible_superentries: Vec<SuperTapeEntry>,
}
impl MachineTranslator {
    pub fn new(machine_description: String) -> AppResult<Self> {
        let input = TranslatorInput::new(machine_description)?;
        let possible_superentries = iproduct!(
            &input.tape_entries.clone(),
            &input.tape_entries.clone(),
            vec![true, false],
            vec![true, false],
            vec![true, false]
        )
        .map(
            |(val1, val2, has_first_head, has_second_head, is_start)| SuperTapeEntry {
                first_letter: val1.clone(),
                second_letter: val2.clone(),
                has_first_tape_head: has_first_head,
                has_second_tape_head: has_second_head,
                is_start: is_start,
            },
        )
        .collect();

        Ok(Self {
            input: input,
            possible_superentries: possible_superentries,
        })
    }

    fn wrap_original_state(state: &State) -> State {
        State(format!("#$ORIGINAL_{}$#", state.value()).to_string())
    }

    fn customized_state(data: &str, values: Vec<String>) -> State {
        State(format!("#$CUSTOMIZED_{}#{}$#", data, values.join("#")).to_string())
    }

    fn initial_start_setup_transitions(&self) -> Vec<Transition> {
        self.input
            .tape_entries
            .iter()
            .map(|e| SuperTapeEntry {
                first_letter: e.clone(),
                second_letter: TapeEntry::BLANK,
                has_first_tape_head: true,
                has_second_tape_head: true,
                is_start: true,
            })
            .map(|se| Transition {
                state_before: Self::customized_state(std_states::START, vec![]),
                state_after: Self::wrap_original_state(&State(std_states::START.to_string())),
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),
                tape_head_move_direction: HeadMoveDirection::Stay,
            })
            .collect()
    }

    fn get_data_to_write(trans: &DoubleTransition) -> Vec<String> {
        vec![
            trans.first_tape_value_after.to_string(),
            trans.second_tape_value_after.to_string(),
            direction_to_string(&trans.first_tape_head_move_direction),
            direction_to_string(&trans.second_tape_head_move_direction),
            trans.state_after.to_string(),
        ]
    }

    fn get_data_to_read_and_write(trans: &DoubleTransition) -> Vec<String> {
        let mut write_data: Vec<String> = Self::get_data_to_write(trans);
        write_data.push(trans.second_tape_value_before.to_string());
        write_data
    }

    fn initialize_two_tape_read(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .filter(|se| se.first_letter == trans.first_tape_value_before)
            .map(|se| Transition {
                state_before: Self::wrap_original_state(&trans.state_before),
                state_after: Self::customized_state(
                    "go_to_start_to_read_q",
                    Self::get_data_to_read_and_write(trans),
                ),
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),
                tape_head_move_direction: HeadMoveDirection::Stay,
            })
            .collect()
    }

    fn go_to_start_to_read_from_q(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "go_to_start_to_read_from_q",
                    Self::get_data_to_read_and_write(trans),
                ),
                state_after: if se.is_start {
                    Self::customized_state(
                        "reach_q_for_read",
                        Self::get_data_to_read_and_write(trans),
                    )
                } else {
                    Self::customized_state(
                        "go_to_start_to_read_from_q",
                        Self::get_data_to_read_and_write(trans),
                    )
                },
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),
                tape_head_move_direction: if se.is_start {
                    HeadMoveDirection::Stay
                } else {
                    HeadMoveDirection::Left
                },
            })
            .collect()
    }

    fn reach_q_for_read(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "reach_q_for_read",
                    Self::get_data_to_read_and_write(trans),
                ),
                state_after: if se.has_second_tape_head {
                    Self::customized_state(
                        "read_value_from_q",
                        Self::get_data_to_read_and_write(trans),
                    )
                } else {
                    Self::customized_state(
                        "reach_q_for_read",
                        Self::get_data_to_read_and_write(trans),
                    )
                },
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),

                tape_head_move_direction: if se.has_second_tape_head {
                    HeadMoveDirection::Stay
                } else {
                    HeadMoveDirection::Right
                },
            })
            .collect()
    }
    fn read_value_from_q(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .filter(|se| se.second_letter == trans.second_tape_value_before)
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "read_value_from_q",
                    Self::get_data_to_read_and_write(trans),
                ),
                state_after: Self::customized_state(
                    "handle_q_head_moved",
                    Self::get_data_to_write(trans),
                ),
                tape_value_before: se.encoded(),
                tape_value_after: SuperTapeEntry {
                    second_letter: trans.second_tape_value_after.clone(),
                    has_second_tape_head: false,
                    ..se.clone()
                }
                .encoded(),
                tape_head_move_direction: trans.second_tape_head_move_direction.clone(),
            })
            .collect()
    }

    fn handle_q_head_moved(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "handle_q_head_moved",
                    Self::get_data_to_write(trans),
                ),
                state_after: Self::customized_state(
                    "go_to_start_to_write_to_p",
                    Self::get_data_to_write(trans),
                ),
                tape_value_before: se.encoded(),
                tape_value_after: SuperTapeEntry {
                    has_second_tape_head: true,
                    ..se.clone()
                }
                .encoded(),
                tape_head_move_direction: HeadMoveDirection::Stay,
            })
            .collect()
    }
    fn go_to_start_to_write_to_p(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "go_to_start_to_write_to_p",
                    Self::get_data_to_write(trans),
                ),
                state_after: if se.is_start {
                    Self::customized_state("reach_p_for_write", Self::get_data_to_write(trans))
                } else {
                    Self::customized_state(
                        "go_to_start_to_write_to_p",
                        Self::get_data_to_write(trans),
                    )
                },
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),
                tape_head_move_direction: if se.is_start {
                    HeadMoveDirection::Stay
                } else {
                    HeadMoveDirection::Left
                },
            })
            .collect()
    }
    fn reach_p_for_write(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "reach_q_for_read",
                    Self::get_data_to_write(trans),
                ),
                state_after: if se.has_first_tape_head {
                    Self::customized_state("write_value_to_p", Self::get_data_to_write(trans))
                } else {
                    Self::customized_state("reach_p_for_write", Self::get_data_to_write(trans))
                },
                tape_value_before: se.encoded(),
                tape_value_after: se.encoded(),

                tape_head_move_direction: if se.has_first_tape_head {
                    HeadMoveDirection::Stay
                } else {
                    HeadMoveDirection::Right
                },
            })
            .collect()
    }
    fn write_value_to_p(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .filter(|se| se.second_letter == trans.second_tape_value_before)
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "write_value_to_p",
                    Self::get_data_to_write(trans),
                ),
                state_after: Self::customized_state(
                    "handle_p_head_moved",
                    Self::get_data_to_write(trans),
                ),
                tape_value_before: se.encoded(),
                tape_value_after: SuperTapeEntry {
                    first_letter: trans.first_tape_value_after.clone(),
                    has_first_tape_head: false,
                    ..se.clone()
                }
                .encoded(),
                tape_head_move_direction: trans.first_tape_head_move_direction.clone(),
            })
            .collect()
    }

    fn handle_p_head_moved(&self, trans: &DoubleTransition) -> Vec<Transition> {
        self.possible_superentries
            .iter()
            .map(|se| Transition {
                state_before: Self::customized_state(
                    "handle_p_head_moved",
                    Self::get_data_to_write(trans),
                ),
                state_after: Self::wrap_original_state(&trans.state_after),
                tape_value_before: se.encoded(),
                tape_value_after: SuperTapeEntry {
                    has_first_tape_head: true,
                    ..se.clone()
                }
                .encoded(),
                tape_head_move_direction: HeadMoveDirection::Stay,
            })
            .collect()
    }

    pub fn translate(&self) -> String {
        let start_transitions = self.initial_start_setup_transitions();
        let new_transitions: Vec<Transition> = self
            .input
            .transitions
            .iter()
            .map(|trans| {
                vec![
                    self.initialize_two_tape_read(trans),
                    self.go_to_start_to_read_from_q(trans),
                    self.reach_q_for_read(trans),
                    self.read_value_from_q(trans),
                    self.handle_q_head_moved(trans),
                    self.go_to_start_to_write_to_p(trans),
                    self.reach_p_for_write(trans),
                    self.write_value_to_p(trans),
                    self.handle_p_head_moved(trans),
                ]
            })
            .flatten()
            .flatten()
            .collect();

        let mut all_transitions: Vec<Transition> = vec![];
        all_transitions.extend(start_transitions);
        all_transitions.extend(new_transitions);
        let all_transition_descriptions: Vec<String> = all_transitions
            .iter()
            .map(|trans| trans.to_string())
            .collect();

        all_transition_descriptions.join("\n")
    }
}
