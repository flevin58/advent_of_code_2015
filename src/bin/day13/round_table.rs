use common::error::{AocError, Result};
use itertools::Itertools;

const MYSELF: &str = "Myself";

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
            if line.trim().is_empty() {
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

    pub fn dispositions(&self) -> Vec<Vec<String>> {
        let perms: Vec<Vec<String>> = self
            .attendees
            .iter()
            .permutations(self.attendees.len())
            .map(|p| p.into_iter().cloned().collect())
            .collect();
        perms
    }

    pub fn happiness_of(&self, person: &String, next_to: &String) -> Option<i32> {
        // This is added for Part2 but it's ok also for Part1 because it's always false.
        if *person == MYSELF || next_to == MYSELF {
            return Some(0);
        }

        for h in &self.happiness_list {
            if h.person == *person && h.next_to == *next_to {
                return Some(h.points);
            }
        }
        None
    }

    pub fn get_max_happiness(&self) -> Result<i32> {
        let mut max_points = 0_i32;
        for disp in self.dispositions() {
            let imax = disp.len();
            let mut disp_points = 0_i32;
            for i in 0..imax {
                let person_left = if i > 0 { &disp[i - 1] } else { &disp[imax - 1] };
                let person_right = if i < imax - 1 { &disp[i + 1] } else { &disp[0] };
                let left_points = self.happiness_of(&disp[i], person_left);
                let right_points = self.happiness_of(&disp[i], person_right);
                if left_points.is_none() || right_points.is_none() {
                    return Err(AocError::ParseError(format!(
                        "Bad table disposition: {disp:?}"
                    )));
                }
                disp_points += left_points.unwrap() + right_points.unwrap();
            }
            if disp_points > max_points {
                max_points = disp_points;
            }
        }
        Ok(max_points)
    }

    pub fn get_max_happiness_with_me(&mut self) -> Result<i32> {
        self.attendees.push(MYSELF.to_string());
        let mut max_points = 0_i32;
        for disp in self.dispositions() {
            let imax = disp.len();
            let mut disp_points = 0_i32;
            for i in 0..imax {
                let person_left = if i > 0 { &disp[i - 1] } else { &disp[imax - 1] };
                let person_right = if i < imax - 1 { &disp[i + 1] } else { &disp[0] };
                let left_points = self.happiness_of(&disp[i], person_left);
                let right_points = self.happiness_of(&disp[i], person_right);
                if left_points.is_none() || right_points.is_none() {
                    return Err(AocError::ParseError(format!(
                        "Bad table disposition: {disp:?}"
                    )));
                }
                disp_points += left_points.unwrap() + right_points.unwrap();
            }
            if disp_points > max_points {
                max_points = disp_points;
            }
        }
        Ok(max_points)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_max_happiness() {
        let input = r#"
            Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Alice would lose 2 happiness units by sitting next to David.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Bob would lose 63 happiness units by sitting next to David.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob.
            Carol would gain 55 happiness units by sitting next to David.
            David would gain 46 happiness units by sitting next to Alice.
            David would lose 7 happiness units by sitting next to Bob.
            David would gain 41 happiness units by sitting next to Carol.
        "#;
        let rt = RoundTable::from_input(&input.to_string()).unwrap();
        let max_points = rt.get_max_happiness().unwrap();
        assert_eq!(max_points, 330);
    }
}
