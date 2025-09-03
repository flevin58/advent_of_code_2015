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
    use test_case::test_case;

    #[test_case("abcdef609043", "000001dbbfa3a5c83a2d506429c7b00e"; "t1")]
    fn md5_digest(input: &str, expected: &str) {
        let digest = md5::compute(input);
        let hash_string = format!("{:x}", digest);
        assert_eq!(hash_string, expected);
    }

    #[test_case("abcdef", 5, 609043; "t1")]
    #[test_case("pqrstuv", 5, 1048970; "t2")]
    fn lowest_number_with_n_leading_zeros(input: &str, zeroes: usize, expected: usize) {
        let result = super::lowest_number_with_n_leading_zeros(input, zeroes);
        assert_eq!(result, expected);
    }
}
