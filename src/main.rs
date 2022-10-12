use clap::Parser;
use std::process::exit;

mod convert;
mod eval;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[clap(author, version)]
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
        eprintln!("No input provided");
        exit(1);
    }

    let result = match cli.convert {
        true => convert::convert(input),
        false => match eval::eval_shunting(input) {
            Ok(r) => r,
            Err(e) => {
                eprint!("{}", e);
                std::process::exit(1);
            }
        },
    };
    println!("{}", result);
}
