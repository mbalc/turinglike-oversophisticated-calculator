use crate::machine::component::transition::Transition;
use crate::types::*;

pub fn transitions_from_file(file_path: String) -> AppResult<Vec<Transition>> {
    let machine_description = std::fs::read_to_string(file_path)?;
    // dbg!("loaded description:\n{}", &machine_description);

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
