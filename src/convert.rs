use regex::Regex;
use rink_core::{one_line, simple_context};
use std::process::exit;

/// Convert the dimensioned unit to a different unit
pub fn convert(input: &str) -> String {
    let re = Regex::new(
        r"(?P<val>-?\d+\.?\d*)\s?(?P<from_unit>[a-zA-Z\s/\*°^\d]+)\s(->?|to)\s(?P<to_unit>[a-zA-Z\s/\*°^\d]+)",
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
    let expression = format!(
        "{} {} -> {}",
        value,
        get_unit_alias(from_unit),
        get_unit_alias(to_unit)
    );

    // rink
    let mut ctx = simple_context().unwrap();
    match one_line(&mut ctx, &expression) {
        Ok(r) => clean_output(r),
        Err(_) => {
            eprintln!("Units are invalid");
            exit(1);
        }
    }
}

/// The rink library doesn't like the abbreviation "in" for "inch". This fixes that.
fn get_unit_alias(unit: &str) -> String {
    let mut new_unit: String = unit.to_string();
    let aliases = vec![("in", "inch")];
    aliases.iter().for_each(|a| {
        let regex_str = format!(r"\b{}\b", a.0);
        let re = Regex::new(&regex_str).unwrap();
        new_unit = re.replace_all(&new_unit, a.1).to_string();
    });
    new_unit
}

/// Remove the extras from rink's post-conversion output
fn clean_output(output: String) -> String {
    // Trim trailing unit type
    let mut new_output = output.split('(').next().unwrap().trim().to_string();
    // Remove `approx.`
    new_output = new_output.replace("approx. ", "");
    // Remove leading fraction
    let re = Regex::new(r"^-?\d+/\d+, ").unwrap();
    new_output = re.replace(&new_output, "").to_string();
    new_output
}

#[test]
fn test_eval_conversion() {
    assert_eq!(convert("1 ft -> inch"), "12 inch");
    assert_eq!(convert("1 mi -> ft"), "5280 foot");
    assert_eq!(convert("10cm -> m"), "0.1 meter");
    assert_eq!(convert("-40 degC -> degF"), "-40 °F");
    assert_eq!(convert("100 kph -> mph"), "62.13711 mph");
    assert_eq!(convert("100 km/hr -> mi/hr"), "62.13711 mile / hour");
    assert_eq!(convert("1 ft - inch"), "12 inch");
    assert_eq!(convert("1 ft to inch"), "12 inch");
    assert_eq!(convert("12 in -> ft"), "1 foot");
    assert_eq!(convert("12 in/s -> mph"), "0.6818181 mph");
    assert_eq!(convert("-12 in/s -> mph"), "-0.6818181 mph");
    assert_eq!(convert("12 in / s -> mph"), "0.6818181 mph");
    assert_eq!(convert("12 ft * lbf -> N * m"), "16.26981 meter newton");
    assert_eq!(convert("1400 ft^2 -> m^2"), "130.0642 meter^2");
}

#[test]
fn test_get_unit_alias() {
    assert_eq!(&get_unit_alias("in"), "inch");
    assert_eq!(&get_unit_alias("in / sec"), "inch / sec");
    assert_eq!(&get_unit_alias("in/sec"), "inch/sec");
}
