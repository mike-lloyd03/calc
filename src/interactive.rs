use std::fmt::Display;

use super::convert::convert;
use super::eval::eval_shunting;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

/// Provides the REP loop for interactive mode
pub fn repl() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let prompt = format!("{} ", ">".red());

    println!("{}", "Calc Interactive Editor".blue().bold());

    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                if line.to_lowercase() == "quit" || line.to_lowercase() == "exit" || line == ":q" {
                    break;
                } else if line.to_lowercase().starts_with("convert ") {
                    rl.add_history_entry(&line);

                    let lower = line.to_lowercase();
                    let (_, input) = lower.split_at("connect ".len());
                    let result = convert(input).unwrap_or_else(format_error);

                    println!("{}", result);
                } else {
                    rl.add_history_entry(&line);

                    let result = eval_shunting(&line).unwrap_or_else(format_error);

                    println!("{}", result);
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn format_error(e: impl Display) -> String {
    e.to_string().yellow().to_string()
}
