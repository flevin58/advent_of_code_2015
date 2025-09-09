use common::error::Result;

use crate::prism::Prism;

fn total_ribbon(single_line: &str) -> Result<u32> {
    let prism = Prism::from_str(single_line)?;
    Ok(prism.total_ribbon())
}

pub fn run() -> Result<()> {
    let input = common::read_input(2)?;
    let total_sum: u32 = input
        .lines()
        .try_fold(0_u32, |acc, line| total_ribbon(line).map(|v| acc + v))?;
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
        let prism = Prism::from_str(measures).unwrap();
        let ribbon = prism.smallest_perimeter();
        assert_eq!(ribbon, expected);
    }

    #[test_case("2x3x4", 24; "measure 2x3x4")]
    #[test_case("1x1x10", 10; "measure 1x1x10")]
    fn perfect_bow(measures: &str, expected: u32) {
        let prism = Prism::from_str(measures).unwrap();
        let ribbon = prism.volume();
        assert_eq!(ribbon, expected);
    }

    #[test_case("2x3x4", 34; "measure 2x3x4")]
    #[test_case("1x1x10", 14; "measure 1x1x10")]
    fn total_ribbon(measures: &str, expected: u32) {
        let prism = Prism::from_str(measures).unwrap();
        let ribbon = prism.total_ribbon();
        assert_eq!(ribbon, expected);
    }
}
