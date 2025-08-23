use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn new_from_direction(self, direction: char) -> Self {
        let (mut x, mut y) = (self.0, self.1);
        match direction {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("Invalid direction"),
        }
        Self(x, y)
    }
}

pub fn visited_houses(directions: &str) -> usize {
    let mut houses = HashSet::<Pos>::new();
    let mut house = Pos(0, 0);
    houses.insert(house);
    for direction in directions.chars() {
        house = house.new_from_direction(direction);
        houses.insert(house);
    }
    houses.len()
}

pub fn visited_houses_with_robot(directions: &str) -> usize {
    let mut houses = HashSet::<Pos>::new();
    let mut santa_house = Pos(0, 0);
    let mut robot_house = Pos(0, 0);
    houses.insert(santa_house);
    houses.insert(robot_house);
    for (i, direction) in directions.chars().enumerate() {
        if i % 2 == 0 {
            santa_house = santa_house.new_from_direction(direction);
            houses.insert(santa_house);
        } else {
            robot_house = robot_house.new_from_direction(direction);
            houses.insert(robot_house);
        }
    }
    houses.len()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_house() {
        use super::Pos;
        type TestCase = (char, Pos);
        const TEST_CASES: [TestCase; 4] = [
            ('>', Pos(1, 0)),
            ('<', Pos(-1, 0)),
            ('^', Pos(0, 1)),
            ('v', Pos(0, -1)),
        ];

        for tc in TEST_CASES {
            let start = Pos(0, 0);
            let end = start.new_from_direction(tc.0);
            assert_eq!(end, tc.1);
        }
    }

    #[test]
    fn test_visited_houses() {
        type TestCase = (&'static str, usize);
        const TEST_CASES: [TestCase; 3] = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for tc in TEST_CASES {
            let result = super::visited_houses(tc.0);
            assert_eq!(result, tc.1);
        }
    }

    #[test]
    fn test_visited_houses_with_robot() {
        type TestCase = (&'static str, usize);
        const TEST_CASES: [TestCase; 3] = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for tc in TEST_CASES {
            let result = super::visited_houses_with_robot(tc.0);
            assert_eq!(result, tc.1);
        }
    }
}
