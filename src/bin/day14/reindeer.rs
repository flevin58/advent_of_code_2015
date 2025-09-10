use anyhow::{Result, bail};
use std::cmp::max;

#[derive(Debug)]
pub struct Reindeer {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    #[allow(dead_code)]
    secs: i32,
    dist: i32,
    points: i32,
}

impl Reindeer {
    pub fn from_str(s: &str) -> Result<Self> {
        let parts = s
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if parts.len() != 15 {
            bail!("Found {} parts instead of 15 in '{}'", parts.len(), s);
        }
        let speed = parts[3].parse::<i32>()?;
        let fly_time = parts[6].parse::<i32>()?;
        let rest_time = parts[13].parse::<i32>()?;
        Ok(Self {
            name: parts[0].clone(),
            speed,
            fly_time,
            rest_time,
            secs: 0,
            dist: 0,
            points: 0,
        })
    }

    // |...fly...|... rest ...|...fly...|...rest...|
    pub fn distance_after(&self, secs: i32) -> i32 {
        let totd = secs / (self.fly_time + self.rest_time);
        let rels = secs % (self.fly_time + self.rest_time);
        (totd + rels) * self.fly_time
    }

    #[cfg(windows)]
    pub fn distance_after(&self, n: i32) -> i32 {
        let mut secs = n;
        let mut tot_distance = 0;

        loop {
            if secs >= self.fly_time {
                tot_distance += self.fly_time * self.speed;
                secs -= self.fly_time;
            } else {
                return tot_distance;
            }

            if secs >= self.rest_time {
                secs -= self.rest_time;
            } else {
                return tot_distance;
            }
        }
    }
}

pub struct ReindeerList {
    list: Vec<Reindeer>,
}

impl ReindeerList {
    pub fn from_str(input: &str) -> Result<Self> {
        let mut v = Vec::<Reindeer>::new();
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let rd = Reindeer::from_str(line)?;
            v.push(rd);
        }
        Ok(Self { list: v })
    }

    pub fn winning_distance_after(self, secs: i32) -> i32 {
        let mut winning_distance = 0;
        for rd in &self.list {
            let d = rd.distance_after(secs);
            if d > winning_distance {
                winning_distance = d;
            }
        }
        winning_distance
    }

    pub fn max_winning_points_after(&mut self, secs: i32) -> i32 {
        for _ in 0..secs {
            // Advance reindeers by 1 second and get max dist
            let mut max_dist = 0;
            for deer in self.list.iter_mut() {
                deer.dist += deer.distance_after(1);
                max_dist = max(max_dist, deer.dist);
            }
            // Add 1 point to each winning deer
            for deer in self.list.iter_mut() {
                if deer.dist == max_dist {
                    deer.points += 1;
                }
            }
        }
        // Return the max points among all deers
        self.list.iter().map(|deer| deer.dist).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = r#"
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "#;

    #[test_case(1000, 1120, "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."; "tc1")]
    #[test_case(1000, 1056, "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."; "tc2")]
    fn test_distance_after(secs: i32, expected: i32, input: &str) {
        let rdv = ReindeerList::from_str(input).unwrap();
        let d1 = rdv.list[0].distance_after(secs);
        assert_eq!(d1, expected);
    }

    #[test]
    fn winning_distance() {
        let rdv = ReindeerList::from_str(INPUT).unwrap();
        let w = rdv.winning_distance_after(1000);
        assert_eq!(w, 1120);
    }
}
