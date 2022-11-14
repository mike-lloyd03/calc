use anyhow::{Error, Result};
use fancy_regex::Regex;
use rustyard::ShuntingYard;

/// Uses the shunting yard algorithm to solve the given input equation
pub fn eval_shunting(equation: &str) -> Result<String> {
    let mut sy = ShuntingYard::new();

    println!("equation before: {}", equation);

    let equation = &prepend_zero(equation);

    println!("equation after: {}", equation);

    let result = sy
        .calculate(equation)
        .map_err(|e| Error::msg(e.join("; ")))?;

    Ok(format!("{}", result))
}

pub fn prepend_zero(equation: &str) -> String {
    let re = Regex::new(r"(?<!\d)\.").expect("Regex should compile");
    re.replace_all(equation, "0.").to_string()
}

#[cfg(test)]
mod eval_tests {
    use super::*;

    #[test]
    fn simple_addition() -> Result<()> {
        assert_eq!(eval_shunting("1 + 2")?, "3");
        Ok(())
    }

    #[test]
    fn multiple_addition() -> Result<()> {
        assert_eq!(eval_shunting("1 + 2 + 3")?, "6");
        Ok(())
    }

    #[test]
    fn simple_division() -> Result<()> {
        assert_eq!(eval_shunting("1 / 2")?, "0.5");
        Ok(())
    }

    #[test]
    fn simple_multiplication() -> Result<()> {
        assert_eq!(eval_shunting("1 * 2")?, "2");
        Ok(())
    }

    #[test]
    fn simple_subtraction() -> Result<()> {
        assert_eq!(eval_shunting("1 - 2")?, "-1");
        Ok(())
    }

    #[test]
    fn dividing_floats() -> Result<()> {
        assert_eq!(eval_shunting("10.2 / 2.5")?, "4.08");
        Ok(())
    }

    #[test]
    fn exponents() -> Result<()> {
        assert_eq!(eval_shunting("2 ^ 8")?, "256");
        Ok(())
    }

    #[test]
    fn exponential_fractions() -> Result<()> {
        assert_eq!(eval_shunting("81 ^ (1/2)")?, "9");
        Ok(())
    }

    #[test]
    fn square_root() -> Result<()> {
        assert_eq!(eval_shunting("sqrt(81)")?, "9");
        Ok(())
    }

    #[test]
    fn leading_decimal() -> Result<()> {
        assert_eq!(eval_shunting(".06 * 24")?, "1.44");
        assert_eq!(eval_shunting("24 *.06")?, "1.44");
        assert_eq!(eval_shunting("24 + .06")?, "24.06");
        Ok(())
    }
}
