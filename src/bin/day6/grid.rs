const GRID_SIZE: usize = 1000;

struct Pos(usize, usize);

impl Pos {
    // Constructs a Pos from a string like "0,0"
    fn from_coords(str_pos: &str) -> Self {
        let parts: Vec<usize> = str_pos
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Pos(parts[0], parts[1])
    }
}
struct Rect(Pos, Pos);

impl Rect {
    fn from_coords(start: &str, end: &str) -> Self {
        let top_left = Pos::from_coords(start);
        let bottom_right = Pos::from_coords(end);
        Rect(top_left, bottom_right)
    }
}
pub struct Grid {
    with_brightness: bool,
    lights: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(with_brightness: bool) -> Self {
        Grid {
            with_brightness,
            lights: vec![vec![0; GRID_SIZE]; GRID_SIZE],
        }
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                self.lights[i][j] = 0;
            }
        }
    }

    pub fn apply_action(&mut self, s: &str) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "turn" => match parts[1] {
                "on" => {
                    let rect = Rect::from_coords(parts[2], parts[4]);
                    self.lights_on(rect);
                }
                "off" => {
                    let rect = Rect::from_coords(parts[2], parts[4]);
                    self.lights_off(rect);
                }
                _ => panic!("Invalid action"),
            },
            "toggle" => {
                let rect = Rect::from_coords(parts[1], parts[3]);
                self.toggle_lights(rect);
            }
            _ => panic!("Invalid action"),
        }
    }

    fn lights_on(&mut self, rect: Rect) {
        for i in rect.0.0..=rect.1.0 {
            for j in rect.0.1..=rect.1.1 {
                if self.with_brightness {
                    self.lights[i][j] += 1;
                } else {
                    self.lights[i][j] = 1;
                }
            }
        }
    }

    fn lights_off(&mut self, rect: Rect) {
        for i in rect.0.0..=rect.1.0 {
            for j in rect.0.1..=rect.1.1 {
                if self.with_brightness {
                    if self.lights[i][j] > 0 {
                        self.lights[i][j] -= 1;
                    }
                } else {
                    self.lights[i][j] = 0;
                }
            }
        }
    }

    fn toggle_lights(&mut self, rect: Rect) {
        for i in rect.0.0..=rect.1.0 {
            for j in rect.0.1..=rect.1.1 {
                if self.with_brightness {
                    self.lights[i][j] += 2;
                } else {
                    self.lights[i][j] = if self.lights[i][j] == 1 { 0 } else { 1 };
                }
            }
        }
    }

    pub fn sum_brightness(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum()
    }

    pub fn count_lights_on(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&light| light == 1).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, Pos, Rect};

    #[test]
    fn test_pos_from_coords() {
        let pos = Pos::from_coords("10,20");
        assert_eq!(pos.0, 10);
        assert_eq!(pos.1, 20);
    }

    #[test]
    fn test_rect_from_coords() {
        let rect = Rect::from_coords("0,0", "999,999");
        assert_eq!(rect.0.0, 0);
        assert_eq!(rect.0.1, 0);
        assert_eq!(rect.1.0, 999);
        assert_eq!(rect.1.1, 999);
    }

    #[test]
    fn test_grid_operations() {
        let mut grid = Grid::new(false);
        grid.apply_action("turn on 0,0 through 999,999");
        assert_eq!(grid.count_lights_on(), 1_000_000);

        grid.apply_action("toggle 0,0 through 999,0");
        assert_eq!(grid.count_lights_on(), 999_000);

        grid.apply_action("turn off 499,499 through 500,500");
        assert_eq!(grid.count_lights_on(), 998_996);
    }

    #[test]
    fn test_brightness_operations() {
        let mut grid = Grid::new(true);
        grid.apply_action("turn on 0,0 through 0,0");
        assert_eq!(grid.sum_brightness(), 1);

        grid.clear();
        grid.apply_action("toggle 0,0 through 999,999");
        assert_eq!(grid.sum_brightness(), 2_000_000);
    }
}
