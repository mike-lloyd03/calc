use clap::Parser;
use eval::eval;
use regex::Regex;
use rink_core::{one_line, simple_context};
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
    let input: &str = &cli.input.join(" ");
    if input.is_empty() {
        eprintln!("No input provided");
        exit(1);
    }

    let result = match cli.convert {
        true => eval_conversion(input),
        false => eval_equation(input),
    };
    println!("{}", result);
}

fn eval_equation(equation: &str) -> String {
    if !validate_equation(equation) {
        eprintln!("Invalid characters in input");
        exit(1);
    }

    match eval(equation) {
        Ok(result) => result.to_string(),
        Err(_) => {
            eprintln!("Unable to parse input");
            exit(1);
        }
    }
}

fn validate_equation(input: &str) -> bool {
    let re = Regex::new(r"[\d\+\-\*/\s\(\)\.]").expect("invalid regex");
    input.chars().all(|c| re.is_match(&c.to_string()))
}

fn eval_conversion(input: &str) -> String {
    let re = Regex::new(
        r"(?P<val>-?\d+\.?\d*)\s?(?P<from_unit>[a-zA-Z/°]+)\s(->?|to)\s(?P<to_unit>[a-zA-Z/°]+)",
    )
    .expect("invalid regex");
    let caps = match re.captures(input) {
        Some(c) => c,
        None => {
            eprintln!("Unable to parse input. Conversion strings should be in the form: '<Value> <Unit> -> <Unit>' (e.g. '12 ft -> m')");
            exit(1);
        }
    };
    let value = caps.name("val").unwrap().as_str();
    let from_unit = caps.name("from_unit").unwrap().as_str();
    let to_unit = caps.name("to_unit").unwrap().as_str();
    let expression = format!("{} {} -> {}", value, from_unit, to_unit);

    // rink
    let mut ctx = simple_context().unwrap();
    match one_line(&mut ctx, &expression) {
        Ok(r) => r.split('(').next().unwrap().trim().to_string(),
        Err(_) => {
            eprintln!("Units are invalid");
            exit(1);
        }
    }
}

#[test]
fn test_validate_equation() {
    assert!(validate_equation("1 + 2"));
    assert!(validate_equation("1 + 2 + 3"));
    assert!(validate_equation("1 / 2"));
    assert!(validate_equation("10.2 / 2.5"));
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
    assert_eq!(eval_equation("10.2 / 2.5"), "4.08");
}

#[test]
fn test_eval_conversion() {
    assert_eq!(eval_conversion("1 ft -> inch"), "12 inch");
    assert_eq!(eval_conversion("1 mi -> ft"), "5280 foot");
    assert_eq!(eval_conversion("10cm -> m"), "0.1 meter");
    assert_eq!(eval_conversion("-40 degC -> degF"), "-40 °F");
    assert_eq!(eval_conversion("100 kph -> mph"), "approx. 62.13711 mph");
    assert_eq!(
        eval_conversion("100 km/hr -> mi/hr"),
        "approx. 62.13711 mile / hour"
    );
    assert_eq!(eval_conversion("1 ft - inch"), "12 inch");
    assert_eq!(eval_conversion("1 ft to inch"), "12 inch");
}
