use common::read_input;

pub fn index_of_basement(s: String) -> usize {
    let mut floor: i16 = 0;
    for (index, ch) in s.chars().enumerate() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 {
            return index + 1;
        }
    }
    0
}

pub fn run() {
    let input_data = read_input!("day1");
    let index = index_of_basement(input_data.to_owned());
    println!("Index of basement = {index}");
}

#[cfg(test)]
mod tests {
    type TestCase = (&'static str, usize);

    const TEST_CASES: [TestCase; 2] = [(")", 1), ("()())", 5)];

    #[test]
    fn test_index_of_basement() {
        for tc in TEST_CASES {
            let result = super::index_of_basement(tc.0.into());
            assert_eq!(result, tc.1);
        }
    }
}
