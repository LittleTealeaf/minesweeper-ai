use std::{fmt::Display, iter::once};

use itertools::chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn iter_range(
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
    ) -> impl Iterator<Item = Self> {
        (min_x..max_x).flat_map(move |x| (min_y..max_y).map(move |y| Self::new(x, y)))
    }

    pub fn dist_step(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn dist_max(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        let Self { x, y } = *self;
        chain!(
            [
                Self::new(x + 1, y),
                Self::new(x, y + 1),
                Self::new(x + 1, y + 1)
            ],
            (x > 0)
                .then(|| [Point::new(x - 1, y), Point::new(x - 1, y + 1)])
                .into_iter()
                .flatten(),
            (y > 0)
                .then(|| [Point::new(x, y - 1), Point::new(x + 1, y - 1)])
                .into_iter()
                .flatten(),
            (x > 0 && y > 0)
                .then(|| once(Point::new(x - 1, y - 1)))
                .into_iter()
                .flatten()
        )
    }
}
