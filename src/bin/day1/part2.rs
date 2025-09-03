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
    use test_case::test_case;

    #[test_case(")", 1; "t1")]
    #[test_case("()())", 5; "t2")]
    fn index_of_basement(directions: &str, expected: usize) {
        let result = super::index_of_basement(directions.into());
        assert_eq!(result, expected);
    }
}
