#[macro_use]
extern crate scan_fmt;
extern crate derive_more;

mod machine;
mod types;

use crate::types::*;
use machine::classic::ClassicMachine;

struct Arguments {
    machine_description: String,
    execution_limit: Number,
}

fn print_usage_message() {
    println!(
        "Usage: {} [machine_description_file] [execution_limit]",
        std::env::args()
            .nth(0)
            .unwrap_or("./interpreter".to_string())
    )
}

fn parse_cmd_arguments() -> AppResult<Arguments> {
    let cmd_args: Vec<String> = std::env::args().collect();

    if cmd_args.len() != 3 {
        eprintln!(
            "error: expected 2 arguments but found {}",
            cmd_args.len() - 1
        );
        print_usage_message();
        std::process::exit(1);
    }

    Ok(Arguments {
        machine_description: std::fs::read_to_string(&cmd_args[1])?,
        execution_limit: cmd_args[2].parse::<Number>()?,
    })
}

fn main() -> Result<(), AppError> {
    let args = parse_cmd_arguments()?;
    let mut stdin = String::new();
    std::io::stdin().read_line(&mut stdin)?;

    let tape_content = stdin.trim().to_string();

    let machine =
        ClassicMachine::new(args.machine_description, args.execution_limit, tape_content)?;

    machine.run_with_limit();

    Ok(())
}
