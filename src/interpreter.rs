#[macro_use]
extern crate scan_fmt;

mod machine;
mod types;

use machine::classic::ClassicMachine;
use types::*;

struct Arguments {
    input_file: String,
    execution_limit: Number,
}

fn print_usage_message() {
    println!(
        "Usage: {} [input_file] [execution_limit]",
        std::env::args().next().unwrap_or("ala".to_string())
    )
}

fn parse_cmd_arguments() -> Arguments {
    let cmd_args: Vec<String> = std::env::args().collect();

    if cmd_args.len() != 3 {
        eprintln!(
            "error: expected 2 arguments but found {}",
            cmd_args.len() - 1
        );
        print_usage_message();
        std::process::exit(1);
    }

    Arguments {
        input_file: cmd_args[1].clone(),
        execution_limit: cmd_args[2].parse::<Number>().unwrap_or_default(),
    }
}

fn main() -> Result<(), AppError> {
    let args = parse_cmd_arguments();
    let stdin = "alamakota".to_string();

    let machine = ClassicMachine::from_file(stdin, args.input_file)?;
    dbg!(&machine);

    machine.run_with_limit(args.execution_limit);

    Ok(())
}
