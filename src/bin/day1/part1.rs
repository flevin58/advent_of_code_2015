use common::read_input;

pub fn floor_number_from_string(s: String) -> isize {
    let mut floor: isize = 0;
    for ch in s.chars() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }
    floor
}

pub fn run() {
    let input_data = read_input!("day1");
    let floor = floor_number_from_string(input_data.to_owned());
    println!("Floor = {floor}");
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

    fn floor_number_from_string(direction: &str, expected: isize) {
        let result = super::floor_number_from_string(direction.into());
        assert_eq!(result, expected);
    }
}
