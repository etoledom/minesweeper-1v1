use rand::RngExt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn random_between(range_x: std::ops::Range<usize>, range_y: std::ops::Range<usize>) -> Point {
        let mut rng = rand::rng();
        Point {
            x: rng.random_range(range_x),
            y: rng.random_range(range_y),
        }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
}
