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
    type TestCase = (&'static str, isize);

    const TEST_CASES: [TestCase; 9] = [
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ];

    #[test]
    fn test_floor_number_from_string() {
        for tc in TEST_CASES {
            let result = super::floor_number_from_string(tc.0.into());
            assert_eq!(result, tc.1);
        }
    }
}
