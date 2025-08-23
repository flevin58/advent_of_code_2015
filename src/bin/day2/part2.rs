use crate::prism::Prism;
use common::read_input;

fn total_ribbon(single_line: &str) -> u32 {
    let prism = Prism::from_str(single_line);
    prism.total_ribbon()
}

pub fn run() {
    let input_data = read_input!("day2");
    let total_sum: u32 = input_data.lines().map(|line| total_ribbon(line)).sum();
    println!("Total wrapping paper = {}", total_sum);
}

#[cfg(test)]
mod tests {
    use crate::prism::Prism;

    type TestCase = (&'static str, u32, u32, u32);

    const TEST_CASES: [TestCase; 2] = [("2x3x4", 10, 24, 34), ("1x1x10", 4, 10, 14)];

    #[test]
    fn test_amount_of_ribbon() {
        for tc in TEST_CASES {
            let prism = Prism::from_str(tc.0);
            let ribbon = prism.smallest_perimeter();
            assert_eq!(ribbon, tc.1);
        }
    }

    #[test]
    fn test_perfect_bow() {
        for tc in TEST_CASES {
            let prism = Prism::from_str(tc.0);
            let ribbon = prism.volume();
            assert_eq!(ribbon, tc.2);
        }
    }

    #[test]
    fn test_total_ribbon() {
        for tc in TEST_CASES {
            let prism = Prism::from_str(tc.0);
            let ribbon = prism.total_ribbon();
            assert_eq!(ribbon, tc.3)
        }
    }
}
