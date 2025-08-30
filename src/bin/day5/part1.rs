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

pub fn run() {
    let input_data = common::read_input!("day5");
    let count = input_data.lines().filter(|line| is_nice(line)).count();
    println!("Number of 'nice' strings: {count}");
}

#[cfg(test)]
mod tests {
    type TestCase = (&'static str, bool);

    const TEST_CASES: [TestCase; 5] = [
        ("ugknbfddgicrmopn", true),
        ("aaa", true),
        ("jchzalrnumimnmhp", false),
        ("haegwjzuvuyypxyu", false),
        ("dvszwmarrgswjxmb", false),
    ];

    #[test]
    fn test_is_nice() {
        for tc in TEST_CASES {
            let result = super::is_nice(tc.0);
            assert_eq!(result, tc.1);
        }
    }
}
