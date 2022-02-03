use regex::Regex;
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let n1_str: String = args.nth(1).expect("three arguments are required");
    let operator: String = args.nth(0).expect("an operator must be provided");
    let n2_str: String = args.nth(0).expect("three arguments are required");

    let n1: f32 = n1_str
        .parse::<f32>()
        .expect("first argument must be a number");
    let n2: f32 = n2_str
        .parse::<f32>()
        .expect("second argument must be a number");

    let result = calculate(operator, n1, n2);

    println!("{}", result)
}

fn calculate(operator: String, n1: f32, n2: f32) -> f32 {
    match operator.as_str() {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "*" | "X" | "x" => n1 * n2,
        "/" => n1 / n2,
        _ => panic!("unknown operator: {}", operator),
    }
}

fn parse_parens(equ: &str) -> Vec<&str> {
    let re = Regex::new(r"\([\d\+\-\*/ ]+\)").expect("failed to generate regex");
    let mut matches = Vec::new();
    for m in re.find_iter(equ) {
        matches.push(m.as_str());
    }
    matches
}

#[test]
fn test_parse_parens() {
    // simple
    let equ = "(1 + 2)";
    let expected = vec!["(1 + 2)"];
    let got = parse_parens(equ);
    assert_eq!(expected, got);

    // two groups
    let equ = "(1 + 2) * (8 - 5)";
    let expected = vec!["(1 + 2)", "(8 - 5)"];
    let got = parse_parens(equ);
    assert_eq!(expected, got);

    // nested parens
    let equ = "((1 + 2) * (8 - 5)) / (6 + 3)";
    let expected = vec!["(1 + 2)", "(8 - 5)", "(6 + 3)"];
    let got = parse_parens(equ);
    assert_eq!(expected, got);
}
