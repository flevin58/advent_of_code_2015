use std::{collections::HashSet, io::Read};

pub struct Password {
    current: Vec<u8>,
    bad_chars: Vec<u8>,
}

impl Password {
    //
    // Public Functions
    //
    pub fn from_str(s: &str) -> Self {
        Self {
            current: s.as_bytes().to_vec(),
            bad_chars: vec![b'i', b'o', b'l'],
        }
    }

    pub fn password(&self) -> String {
        String::from_utf8(self.current.clone()).unwrap()
    }

    pub fn next_valid_password(&mut self) -> String {
        self.increase();
        while !self.is_valid_password() {
            self.increase();
        }
        self.password()
    }

    //
    // Password Manipulation Functions
    //
    fn password_to_base26(&self) -> u64 {
        self.current
            .bytes()
            .fold(0, |acc, c| acc * 26 + (c.unwrap() - b'a') as u64)
    }

    fn base26_to_password(&mut self, mut n: u64) {
        let mut chars: Vec<u8> = vec![b'a'; 8];
        for i in (0..8).rev() {
            chars[i] = ((n % 26) + b'a' as u64) as u8;
            n /= 26;
        }
        self.current = chars.clone();
    }

    fn increase(&mut self) {
        let new_number = self.password_to_base26();
        self.base26_to_password(new_number + 1);
    }

    //
    // Error Checking Functions
    //
    fn has_no_bad_letters(&self) -> bool {
        !self.current.iter().any(|b| self.bad_chars.contains(b))
    }

    fn has_consecutive_triplet(&self) -> bool {
        for i in 0..(self.current.len() - 2) {
            let a = self.current[i];
            let b = self.current[i + 1];
            let c = self.current[i + 2];
            if b == a + 1 && c == a + 2 {
                return true;
            }
        }
        false
    }

    fn has_two_pairs(&self) -> bool {
        let mut pairs = HashSet::new();

        for i in 0..(self.current.len() - 1) {
            let a = self.current[i];
            let b = self.current[i + 1];

            if a == b {
                pairs.insert(a);
            }
        }

        pairs.len() >= 2
    }

    fn is_valid_password(&self) -> bool {
        self.current.len() == 8
            && self.has_no_bad_letters()
            && self.has_consecutive_triplet()
            && self.has_two_pairs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("asdfbnyp", true; "tc1")]
    #[test_case("asdibnyp", false; "tc2")]
    #[test_case("asdzbnyo", false; "tc3")]
    fn has_no_bad_letters(password: &str, expected: bool) {
        let pw = Password::from_str(password);
        assert_eq!(pw.has_no_bad_letters(), expected);
    }

    #[test_case("zotabcyy", true; "tc1")]
    #[test_case("asdibnyp", false; "tc2")]
    fn has_consecutive_triplet(password: &str, expected: bool) {
        let pw = Password::from_str(password);
        assert_eq!(pw.has_consecutive_triplet(), expected);
    }

    #[test_case("zotabcyy", true; "tc1")]
    #[test_case("aadibbyp", false; "tc2")]
    fn has_two_pairs(password: &str, expected: bool) {
        let pw = Password::from_str(password);
        assert_eq!(pw.has_consecutive_triplet(), expected);
    }

    #[test_case("hijklmmn", false; "tc1")]
    #[test_case("abbceffg", false; "tc2")]
    #[test_case("abbcegjk", false; "tc3")]
    fn valid_password(password: &str, expected: bool) {
        let pw = Password::from_str(password);
        assert_eq!(pw.is_valid_password(), expected);
    }

    #[test_case("asdfbnyp", "asdfbnyq"; "tc1")]
    #[test_case("abcdefgz", "abcdefha"; "tc2")]
    fn increase(password: &str, expected: &str) {
        let mut pw = Password::from_str(password);
        pw.increase();
        assert_eq!(pw.password(), expected);
    }

    #[test_case("abcdefgh", "abcdffaa"; "tc1")]
    #[test_case("ghijklmn", "ghjaabcc"; "tc2")]
    fn next_valid_password(password: &str, expected: &str) {
        let mut pw = Password::from_str(password);
        assert_eq!(pw.next_valid_password(), expected);
    }
}
