use anyhow::{Error, Result};
use rustyard::ShuntingYard;

/// Uses the shunting yard algorithm to solve the given input equation
pub fn eval_shunting(equation: &str) -> Result<String> {
    let mut sy = ShuntingYard::new();

    let result = sy
        .calculate(equation)
        .map_err(|e| Error::msg(e.join("; ")))?;

    Ok(format!("{}", result))
}

#[test]
fn test_eval_shunting() -> Result<()> {
    assert_eq!(eval_shunting("1 + 2")?, "3");
    assert_eq!(eval_shunting("1 + 2 + 3")?, "6");
    assert_eq!(eval_shunting("1 / 2")?, "0.5");
    assert_eq!(eval_shunting("1 * 2")?, "2");
    assert_eq!(eval_shunting("1 - 2")?, "-1");
    assert_eq!(eval_shunting("10.2 / 2.5")?, "4.08");
    assert_eq!(eval_shunting("2 ^ 8")?, "256");
    assert_eq!(eval_shunting("81 ^ (1/2)")?, "9");
    assert_eq!(eval_shunting("sqrt(81)")?, "9");
    Ok(())
}
