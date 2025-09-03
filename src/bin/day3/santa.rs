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
    use super::Pos;
    use test_case::test_case;

    #[test_case('>', Pos(1, 0); "move right")]
    #[test_case('<', Pos(-1, 0); "move left")]
    #[test_case('^', Pos(0, 1); "move up")]
    #[test_case('v', Pos(0, -1); "move down")]
    fn new_house(direction: char, expected: Pos) {
        let start = Pos(0, 0);
        let end = start.new_from_direction(direction);
        assert_eq!(end, expected);
    }

    #[test_case(">", 2; "t1")]
    #[test_case("^>v<", 4; "t2")]
    #[test_case("^v^v^v^v^v", 2; "t3")]
    fn visited_houses(directions: &str, expected: usize) {
        let result = super::visited_houses(directions);
        assert_eq!(result, expected);
    }

    #[test_case("^v", 3; "t1")]
    #[test_case("^>v<", 3; "t2")]
    #[test_case("^v^v^v^v^v", 11; "t3")]
    fn visited_houses_with_robot(directions: &str, expected: usize) {
        let result = super::visited_houses_with_robot(directions);
        assert_eq!(result, expected);
    }
}
