use eval::eval;
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args();
    args.next();
    let equation: String = args.collect();

    match eval(&equation) {
        Ok(result) => println!("{}", result),
        Err(_) => {
            println!("Unable to parse input");
            exit(1);
        }
    }
}
