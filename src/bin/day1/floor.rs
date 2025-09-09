use common::error::{AocError, Result};

pub fn floor_number_from_string(s: &String) -> Result<i32> {
    let mut floor = 0;
    for ch in s.chars() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            other_ch => return Err(AocError::CharParseError(other_ch)),
        }
    }
    Ok(floor)
}

pub fn index_of_basement(s: &String) -> Result<i32> {
    let mut floor = 0;
    for (index, ch) in s.chars().enumerate() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            other_ch => return Err(AocError::CharParseError(other_ch)),
        };
        if floor == -1 {
            return Ok(index as i32 + 1);
        }
    }
    Err(AocError::NotFound)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("(())", 0; "t1")]
    #[test_case("()()", 0; "t2")]
    #[test_case("(((", 3; "t3")]
    #[test_case("(()(()(", 3; "t4")]
    #[test_case("))(((((", 3; "t5")]
    #[test_case("())", -1; "t6")]
    #[test_case("))(", -1; "t7")]
    #[test_case(")))", -3; "t8")]
    #[test_case(")())())", -3; "t9")]

    fn floor_number_from_string(direction: &str, expected: i32) {
        let result = super::floor_number_from_string(&direction.to_string()).unwrap();
        assert_eq!(result, expected);
    }
    #[test_case(")", 1; "t1")]
    #[test_case("()())", 5; "t2")]
    fn index_of_basement(directions: &str, expected: i32) {
        let result = super::index_of_basement(&directions.into()).unwrap();
        assert_eq!(result, expected);
    }
}
