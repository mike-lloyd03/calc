use clap::Parser;
use std::{fmt::Display, process::exit};

use crate::eval::eval_shunting;

mod convert;
mod eval;
mod interactive;

static ABOUT: &str = r#"
Command line calculator

This tool has three modes: evaluating math expressions, converting between units, and interactive mode.

Evaluating

Most simple math expressions can be given as arguments.

Example:
$ calc 5 + 9
  14

Note: Quotes are necessary when using special characters like `*`, `/`, and `()`.

Converting

Converting between units requires the --convert flag. Units can be converted by entering the number and source unit and the desired unit to convert to.

Examples:
60 mpg -> kph
9.8 m/s to ft/s

Interactive

Launching the app without any arguments goes into interactive mode. This will launch a REPL where you can enter any series of expressions to be evaluating. Converting between units can also be achieved by prefixing any line with `convert` and entering the expression. The REPL can be exited with Ctrl-C, typing "quit", "exit", or ":q".
"#;

#[derive(Parser, Debug)]
#[clap(author, version, about = ABOUT)]
struct Cli {
    /// Use unit conversion mode. Units can be converted by entering the number and source unit and
    /// the desired unit to convert to. Example: `60 mpg -> kph` or `9.8 m/s to ft/s`
    #[clap(short, long)]
    convert: bool,

    /// The equation or conversion to calculate. Example: 12 + 5
    #[clap(num_args(1..))]
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let input: &str = &cli.input.join(" ");
    if input.is_empty() {
        interactive::repl().unwrap_or_else(|e| end(e));
    } else {
        let result = match cli.convert {
            true => convert::convert(input),
            false => eval_shunting(input),
        };
        println!("{}", result.unwrap_or_else(|e| end(e)));
    }
}

fn end(error: impl Display) -> ! {
    eprintln!("{}", error);
    exit(1)
}
