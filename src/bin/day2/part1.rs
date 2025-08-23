use crate::prism::Prism;
use common::read_input;

fn amount_of_wrapping_paper(single_line: &str) -> u32 {
    let prism = Prism::from_str(single_line);
    prism.total_wrapping_paper()
}

pub fn run() {
    let input_data = read_input!("day2");
    let total_sum: u32 = input_data
        .lines()
        .map(|line| amount_of_wrapping_paper(line))
        .sum();
    println!("Total wrapping paper = {}", total_sum);
}

#[cfg(test)]
mod tests {
    type TestCase = (&'static str, u32);

    const TEST_CASES: [TestCase; 2] = [("2x3x4", 58), ("1x1x10", 43)];

    #[test]
    fn test_amount_of_wrapping_paper() {
        for tc in TEST_CASES {
            let result = super::amount_of_wrapping_paper(tc.0);
            assert_eq!(result, tc.1);
        }
    }
}
