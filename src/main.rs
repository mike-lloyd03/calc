use clap::Parser;
use std::process::exit;

mod convert;
mod eval;

/// Run basic calculations or unit conversions
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Cli {
    /// Use unit conversion mode. Units can be converted by entering the number and source unit and
    /// the desired unit to convert to. Example: `60 mpg -> kph` or `9.8 m/s to ft/s`
    #[clap(short, long)]
    convert: bool,

    /// The equation or conversion to calculate. Example: 12 + 5
    #[clap(multiple_values = true)]
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let input: &str = &cli.input.join(" ");
    if input.is_empty() {
        eprintln!("No input provided");
        exit(1);
    }

    let result = match cli.convert {
        true => convert::convert(input),
        false => eval::evaluate(input),
    };
    println!("{}", result);
}
