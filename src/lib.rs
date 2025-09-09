use error::Result;

pub fn read_input(day: u8) -> Result<String> {
    let path = format!("src/bin/day{day}/input.txt");
    let input = std::fs::read_to_string(path)?;
    Ok(input)
}

pub mod error {
    use std::num::ParseIntError;

    #[derive(Debug)]
    pub enum AocError {
        CharParseError(char),
        NumParseError(String),
        ParseError(String),
        IoError(std::io::Error),
        NotFound,
    }

    pub type Result<T> = std::result::Result<T, AocError>;

    impl From<std::io::Error> for AocError {
        fn from(value: std::io::Error) -> Self {
            AocError::IoError(value)
        }
    }

    impl From<ParseIntError> for AocError {
        fn from(value: ParseIntError) -> Self {
            AocError::NumParseError(value.to_string())
        }
    }

    impl std::fmt::Display for AocError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg = match self {
                Self::CharParseError(ch) => format!("Bad character found: '{ch}'"),
                Self::NumParseError(str_num) => format!("Bad number found: '{str_num}'"),
                Self::ParseError(s) => format!("Error parsing: '{s}'"),
                Self::IoError(s) => format!("Io Error: '{}'", s),
                Self::NotFound => String::from("Not Found"),
            };
            write!(f, "{msg}")
        }
    }
    impl std::error::Error for AocError {}
}
