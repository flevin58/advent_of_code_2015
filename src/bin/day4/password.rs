pub fn lowest_number_with_n_leading_zeros(door_id: &str, n: usize) -> usize {
    for i in 0.. {
        let input = format!("{}{}", door_id, i);
        let digest = md5::compute(input);
        let hash_string = format!("{:x}", digest);
        let zeroes = "0".repeat(n);
        if hash_string.starts_with(&zeroes) {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_md5() {
        let input = "abcdef609043";
        let digest = md5::compute(input);
        let hash_string = format!("{:x}", digest);
        assert_eq!(hash_string, "000001dbbfa3a5c83a2d506429c7b00e");
    }

    #[test]
    fn test_lowest_number_with_n_leading_zeros() {
        type TestCase = (&'static str, usize, usize);
        const TEST_CASES: [TestCase; 2] = [("abcdef", 5, 609043), ("pqrstuv", 5, 1048970)];

        for tc in TEST_CASES {
            let result = super::lowest_number_with_n_leading_zeros(tc.0, tc.1);
            assert_eq!(result, tc.2);
        }
    }
}
