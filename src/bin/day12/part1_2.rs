use common::error::{AocError, Result};
use serde_json::Value;

trait Sum {
    fn sum(&self, ignore_red: bool) -> i64;
}

impl Sum for Value {
    fn sum(&self, ignore_red: bool) -> i64 {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::Number(v) => v.as_i64().unwrap_or(0),
            Value::String(_) => 0,
            Value::Array(v) => v.iter().map(|e| e.sum(ignore_red)).sum(),
            Value::Object(m) => {
                let red_keys = m.contains_key("red");
                let red_values = m.values().any(|v| v == "red");
                if ignore_red && (red_keys || red_values) {
                    0
                } else {
                    m.values().map(|v| v.sum(ignore_red)).sum()
                }
            }
        }
    }
}

pub fn run() -> Result<()> {
    let input = common::read_input(12)?;
    let v: Value = serde_json::from_str(&input)
        .map_err(|e| AocError::ParseError(format!("Failed to deserialize JSON: {e}")))?;

    println!("Sum: {}", v.sum(false));
    println!("Sum without red: {}", v.sum(true));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    //
    // Part1 Tests
    //
    #[test_case("[1,2,3]", false, 6; "p1_1")]
    #[test_case(r#"{"a":2,"b":4}"#, false, 6; "p1_2")]
    #[test_case("[[3]]", false, 3; "p1_3")]
    #[test_case(r#"{"a":{"b":4},"c":-1}"#, false, 3; "p1_4")]
    #[test_case(r#"{"a":[-1,1]}"#, false, 0; "p1_5")]
    #[test_case(r#"[-1,{"a":1}]"#, false, 0; "p1_6")]
    #[test_case("[]", false, 0; "p1_7")]
    #[test_case("{}", false, 0; "p1_8")]
    //
    // Part2 Tests
    //
    #[test_case("[1,2,3]", true, 6; "p2_1")]
    #[test_case(r#"[1,{"c":"red","b":2},3]"#, true, 4; "p2_2")]
    #[test_case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, true, 0; "p2_3")]
    #[test_case(r#"[1,"red",5]"#, true, 6; "p2_4")]
    fn test_name(input: &str, ignore_red: bool, expected: i64) {
        let v: Value = serde_json::from_str(input).unwrap();
        assert_eq!(v.sum(ignore_red), expected);
    }
}
