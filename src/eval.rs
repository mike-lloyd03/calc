use eval::eval;
use regex::Regex;
use std::process::exit;

/// Solves the input equation
pub fn evaluate(equation: &str) -> String {
    if !validate_equation(equation) {
        eprintln!("Invalid characters in input");
        exit(1);
    }

    match eval(&equation.to_lowercase().replace('x', "*")) {
        Ok(result) => result.to_string(),
        Err(_) => {
            eprintln!("Unable to parse input");
            exit(1);
        }
    }
}

/// Ensure the input equation doesn't have any invalid characters before it is passed to `eval`
fn validate_equation(input: &str) -> bool {
    let re = Regex::new(r"[\d\+\-\*/\s\(\)\.xX]").expect("invalid regex");
    input.chars().all(|c| re.is_match(&c.to_string()))
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
    assert!(validate_equation("5 x 2"));
    assert!(validate_equation("5 X 2"));

    assert!(!validate_equation("1 + k"));
}

#[test]
fn test_eval_equation() {
    assert_eq!(evaluate("1 + 2"), "3");
    assert_eq!(evaluate("1 + 2 + 3"), "6");
    assert_eq!(evaluate("1 / 2"), "0.5");
    assert_eq!(evaluate("1 * 2"), "2");
    assert_eq!(evaluate("1 - 2"), "-1");
    assert_eq!(evaluate("10.2 / 2.5"), "4.08");
    assert_eq!(evaluate("5 x 2"), "10");
    assert_eq!(evaluate("5 X 2"), "10");
}
