use common::error::{AocError, Result};

#[derive(Clone, Debug)]
pub struct Happiness {
    person: String,
    next_to: String,
    points: i32,
}

impl Happiness {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut parts = s
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if parts.len() != 11 {
            return Err(AocError::ParseError(format!(
                "Found {} parts instead of 11 in '{}'",
                parts.len(),
                s
            )));
        }
        let mut points = match parts[3].parse::<i32>() {
            Ok(value) => value,
            Err(e) => {
                return Err(AocError::ParseError(e.to_string()));
            }
        };
        points *= match parts[2].as_str() {
            "gain" => 1,
            "lose" => -1,
            _ => {
                return Err(AocError::ParseError(format!(
                    "Expected 'gain' or 'lose', found '{}' instead.",
                    parts[2]
                )));
            }
        };
        // Remove trailin 'dot' from next
        parts[10].pop();
        Ok(Self {
            person: parts[0].clone(),
            next_to: parts[10].clone(),
            points,
        })
    }
}

#[derive(Debug)]
pub struct RoundTable {
    attendees: Vec<String>,
    happiness_list: Vec<Happiness>,
}

impl RoundTable {
    pub fn from_input(input: &String) -> Result<Self> {
        let mut rt = Self {
            attendees: Vec::new(),
            happiness_list: Vec::new(),
        };

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let h = Happiness::from_str(line)?;
            rt.happiness_list.push(h.clone());
            if !rt.attendees.contains(&h.person) {
                rt.attendees.push(h.person.clone());
            }
        }
        Ok(rt)
    }
}
