use crate::machine::component::transition::Transition;
use crate::machine::translation::double_transition::DoubleTransition;
use crate::types::*;

fn sanitize_machine_description(description: String) -> Vec<String> {
    description
        .split("\n")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn transitions_from_description(machine_description: String) -> AppResult<Vec<Transition>> {
    let transition_descriptions = sanitize_machine_description(machine_description);

    transition_descriptions
        .iter()
        .map(|x| Transition::from_description(x))
        .collect()
}

pub fn two_tape_transitions_from_description(
    machine_description: String,
) -> AppResult<Vec<DoubleTransition>> {
    let transition_descriptions = sanitize_machine_description(machine_description);

    transition_descriptions
        .iter()
        .map(|x| DoubleTransition::from_description(x))
        .collect()
}
