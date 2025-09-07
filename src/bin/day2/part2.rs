use crate::prism::Prism;

fn total_ribbon(single_line: &str) -> u32 {
    let prism = Prism::from_str(single_line);
    prism.total_ribbon()
}

pub fn run() -> Result<(), String> {
    let input = common::read_input(2)?;
    let total_sum: u32 = input.lines().map(|line| total_ribbon(line)).sum();
    println!("Total wrapping paper = {}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prism::Prism;
    use test_case::test_case;

    #[test_case("2x3x4", 10; "measure 2x3x4")]
    #[test_case("1x1x10", 4; "measure 1x1x10")]
    fn amount_of_ribbon(measures: &str, expected: u32) {
        let prism = Prism::from_str(measures);
        let ribbon = prism.smallest_perimeter();
        assert_eq!(ribbon, expected);
    }

    #[test_case("2x3x4", 24; "measure 2x3x4")]
    #[test_case("1x1x10", 10; "measure 1x1x10")]
    fn perfect_bow(measures: &str, expected: u32) {
        let prism = Prism::from_str(measures);
        let ribbon = prism.volume();
        assert_eq!(ribbon, expected);
    }

    #[test_case("2x3x4", 34; "measure 2x3x4")]
    #[test_case("1x1x10", 14; "measure 1x1x10")]
    fn total_ribbon(measures: &str, expected: u32) {
        let prism = Prism::from_str(measures);
        let ribbon = prism.total_ribbon();
        assert_eq!(ribbon, expected);
    }
}
