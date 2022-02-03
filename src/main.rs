use eval::eval;
use regex::Regex;
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args();
    args.next();
    let equation: String = args.collect();
    if !validate(&equation) {
        println!("Invalid characters in input");
        exit(1);
    }

    match eval(&equation) {
        Ok(result) => println!("{}", result),
        Err(_) => {
            println!("Unable to parse input");
            exit(1);
        }
    }
}

fn validate(input: &String) -> bool {
    let re = Regex::new(r"[\d\+\-\*/\s\(\)]").expect("invalid regex");
    input.chars().all(|c| re.is_match(&c.to_string()))
}
