use crate::prism::Prism;
use anyhow::Result;

fn amount_of_wrapping_paper(single_line: &str) -> Result<u32> {
    let prism = Prism::from_str(single_line)?;
    Ok(prism.total_wrapping_paper())
}

pub fn run() -> Result<()> {
    let input = common::read_input(2)?;
    let total_sum: u32 = input.lines().try_fold(0_u32, |acc, line| {
        amount_of_wrapping_paper(line).map(|v| acc + v)
    })?;
    println!("Total wrapping paper = {}", total_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("2x3x4", 58; "measure 2x3x4")]
    #[test_case("1x1x10", 43; "measure 1x1x10")]
    fn amount_of_wrapping_paper(measures: &str, expected: u32) {
        let result = super::amount_of_wrapping_paper(measures).unwrap();
        assert_eq!(result, expected);
    }
}
