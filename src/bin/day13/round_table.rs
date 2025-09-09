#[derive(Clone, Debug)]
pub struct Happiness {
    person: String,
    next_to: String,
    points: i32,
}

impl Happiness {
    pub fn from_str(s: &str) -> Self {
        let mut parts = s
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(parts.len(), 11, "Badly formatted line");
        let mut points = parts[3].parse::<i32>().expect("Could not parse points");
        points *= match parts[2].as_str() {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("Badly formatted line"),
        };
        // Remove trailin 'dot' from next
        parts[10].pop();
        Self {
            person: parts[0].clone(),
            next_to: parts[10].clone(),
            points,
        }
    }
}

#[derive(Debug)]
pub struct RoundTable {
    attendees: Vec<String>,
    happiness_list: Vec<Happiness>,
}

impl RoundTable {
    pub fn from_input(input: &String) -> Self {
        let mut rt = Self {
            attendees: Vec::new(),
            happiness_list: Vec::new(),
        };

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let h = Happiness::from_str(line);
            rt.happiness_list.push(h.clone());
            if !rt.attendees.contains(&h.person) {
                rt.attendees.push(h.person.clone());
            }
        }
        rt
    }

    pub fn points_when_next_to(&self, person: &String) -> i32 {
        0
    }
}
