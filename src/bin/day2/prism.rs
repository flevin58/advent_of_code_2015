use common::error::Result;
pub struct Prism {
    l: u32,
    w: u32,
    h: u32,
}

impl Prism {
    pub fn from_str(s: &str) -> Result<Self> {
        let dims: Vec<u32> = s.split('x').map(|x| x.parse::<u32>().unwrap()).collect();
        Ok(Prism {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        })
    }

    pub fn surface_area(&self) -> u32 {
        2 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    fn smallest_side_area(&self) -> u32 {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;
        *[side1, side2, side3].iter().min().unwrap()
    }

    pub fn total_wrapping_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side_area()
    }

    pub fn smallest_perimeter(&self) -> u32 {
        let mut sides = vec![self.l, self.w, self.h];
        sides.sort();
        2 * (sides[0] + sides[1])
    }

    pub fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    pub fn total_ribbon(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

#[cfg(test)]
mod tests {
    use super::Prism;

    #[test]
    fn test_prism_from_str() {
        let p = Prism::from_str("2x3x4").unwrap();
        assert_eq!(p.l, 2);
        assert_eq!(p.w, 3);
        assert_eq!(p.h, 4);
    }

    #[test]
    fn test_surface_area() {
        let p = Prism::from_str("2x3x4").unwrap();
        assert_eq!(p.surface_area(), 52);
    }

    #[test]
    fn test_smallest_side_area() {
        let p = Prism::from_str("2x3x4").unwrap();
        assert_eq!(p.smallest_side_area(), 6);
    }

    #[test]
    fn test_smallest_perimeter() {
        let p = Prism::from_str("2x3x4").unwrap();
        assert_eq!(p.smallest_perimeter(), 10);
    }
}
