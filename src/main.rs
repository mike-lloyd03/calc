use clap::Parser;
use eval::eval;
use regex::Regex;
use std::env::args;
use std::process::exit;

/// Run basic calculations or unit conversions
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Cli {
    /// Convert units
    #[clap(short, long)]
    convert: bool,

    #[clap(multiple_values = true)]
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let input: String = cli.input.join(" ");

    let result = eval_equation(&input);
    println!("{}", result);
}

fn eval_equation(equation: &str) -> String {
    if !validate_equation(equation) {
        println!("Invalid characters in input");
        exit(1);
    }

    match eval(equation) {
        Ok(result) => result.to_string(),
        Err(_) => {
            println!("Unable to parse input");
            exit(1);
        }
    }
}

fn validate_equation(input: &str) -> bool {
    let re = Regex::new(r"[\d\+\-\*/\s\(\)]").expect("invalid regex");
    input.chars().all(|c| re.is_match(&c.to_string()))
}

#[test]
fn test_validate_equation() {
    assert!(validate_equation("1 + 2"));
    assert!(validate_equation("1 + 2 + 3"));
    assert!(validate_equation("1 / 2"));
    assert!(validate_equation("1 * 2"));
    assert!(validate_equation("1 - 2"));
    assert!(validate_equation("1+2"));

    assert!(!validate_equation("1 + k"));
}

#[test]
fn test_eval_equation() {
    assert_eq!(eval_equation("1 + 2"), "3");
    assert_eq!(eval_equation("1 + 2 + 3"), "6");
    assert_eq!(eval_equation("1 / 2"), "0.5");
    assert_eq!(eval_equation("1 * 2"), "2");
    assert_eq!(eval_equation("1 - 2"), "-1");
}
