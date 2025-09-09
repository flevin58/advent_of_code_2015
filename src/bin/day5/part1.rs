use common::error::Result;

fn is_nice(s: &str) -> bool {
    // Rule 1: it contains at least three vowels (aeiou only)
    let vowels = "aeiou";
    let vowel_count = s.chars().filter(|c| vowels.contains(*c)).count();
    if vowel_count < 3 {
        return false;
    }

    // Rule 2: it contains at least one letter that appears twice in a row
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    if !has_double {
        return false;
    }

    // Rule 3: it does not contain the strings "ab", "cd", "pq", or "xy"
    let forbidden = ["ab", "cd", "pq", "xy"];
    if forbidden.iter().any(|&f| s.contains(f)) {
        return false;
    }

    // If it passes all the rules, it's nice
    true
}

pub fn run() -> Result<()> {
    let input = common::read_input(5)?;
    let count = input.lines().filter(|line| is_nice(line)).count();
    println!("Number of 'nice' strings: {count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("ugknbfddgicrmopn", true; "t1.1")]
    #[test_case("aaa", true; "t1.2")]
    #[test_case("jchzalrnumimnmhp", false; "t1.3")]
    #[test_case("haegwjzuvuyypxyu", false; "t1.4")]
    #[test_case("dvszwmarrgswjxmb", false; "t1.5")]
    fn is_nice(input: &str, expected: bool) {
        let result = super::is_nice(input);
        assert_eq!(result, expected);
    }
}
