#[macro_use]
extern crate scan_fmt;
extern crate derive_more;

mod machine;
mod types;

use intbits::Bits;
use types::*;

fn print_usage_message() {
    println!(
        "Usage: {} [two_tape_machine_description_file]",
        std::env::args().nth(0).unwrap_or("./translate".to_string())
    )
}
fn parse_cmd_arguments() -> AppResult<String> {
    let cmd_args: Vec<String> = std::env::args().collect();

    if cmd_args.len() != 2 {
        eprintln!(
            "error: expected 1 arguments but found {}",
            cmd_args.len() - 1
        );
        print_usage_message();
        std::process::exit(1);
    }

    Ok(std::fs::read_to_string(&cmd_args[1])?)
}
fn main() -> Result<(), AppError> {
    let machine_description = parse_cmd_arguments()?;
    let translator = machine::translation::translator::MachineTranslator::new(machine_description)?;
    print!("{}\n", translator.translate());

    Ok(())
}
