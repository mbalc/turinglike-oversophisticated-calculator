use crate::machine::component::transition::Transition;
use crate::types::*;

pub fn transitions_from_description(machine_description: String) -> AppResult<Vec<Transition>> {
    let transition_descriptions: Vec<String> = machine_description
        .split("\n")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect();
    // dbg!(&transition_descriptions);

    transition_descriptions
        .iter()
        .map(|x| Transition::from_description(x))
        .collect()
}
