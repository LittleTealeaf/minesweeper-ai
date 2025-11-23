use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Debug, Display},
    iter::once,
};

use itertools::chain;
use rand::random;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
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

    /**
     * Returns the furthest coordinate distance
     */
    pub fn dist(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Point> {
        chain!(
            [
                Point::new(self.x + 1, self.y),
                Point::new(self.x, self.y + 1),
                Point::new(self.x + 1, self.y + 1),
            ]
            .into_iter(),
            (self.x > 0)
                .then(|| [
                    Point::new(self.x - 1, self.y),
                    Point::new(self.x - 1, self.y + 1),
                ])
                .into_iter()
                .flatten(),
            (self.y > 0)
                .then(|| [
                    Point::new(self.x, self.y - 1),
                    Point::new(self.x + 1, self.y - 1)
                ])
                .into_iter()
                .flatten(),
            (self.y > 0 && self.x > 0)
                .then(|| once(Point::new(self.x - 1, self.y - 1)))
                .into_iter()
                .flatten()
        )
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Mine,
    Blank,
    Warn(usize),
    Flag,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Mine => write!(f, "M"),
            Tile::Blank => write!(f, "_"),
            Tile::Warn(n) => write!(f, "{n}"),
            Tile::Flag => write!(f, "F"),
        }
    }
}

pub struct MineSweeper {
    width: usize,
    height: usize,
    mine_count: usize,
    game_status: GameStatus,
    mines: HashSet<Point>,
    flags: HashSet<Point>,
    revealed: HashMap<Point, Tile>,
}

impl Display for MineSweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x, y);
                if self.revealed.contains_key(&p) {
                    write!(f, "{}", self.get_tile(p).unwrap())?;
                } else if self.flagged().contains(&p) {
                    write!(f, "F")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl MineSweeper {
    pub fn new(
        width: usize,
        height: usize,
        mine_count: usize,
    ) -> Result<Self, NewMinesweeperError> {
        if width == 0 {
            return Err(NewMinesweeperError::InvalidWidth);
        } else if height == 0 {
            return Err(NewMinesweeperError::InvalidHeight);
        } else if (height * width) < mine_count {
            return Err(NewMinesweeperError::TooManyMines {
                area: height * width,
                mines: mine_count,
            });
        }

        Ok(Self {
            height,
            width,
            game_status: GameStatus::Blank,
            mine_count,
            mines: HashSet::new(),
            flags: HashSet::new(),
            revealed: HashMap::new(),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn all_points(&self) -> impl Iterator<Item = Point> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| Point::new(x, y)))
    }

    pub fn revealed(&self) -> &HashMap<Point, Tile> {
        &self.revealed
    }

    pub fn flagged(&self) -> &HashSet<Point> {
        &self.flags
    }

    pub fn is_valid_point(&self, coord: Point) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn get_status(&self) -> GameStatus {
        self.game_status
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.game_status, GameStatus::InProgress)
    }

    fn generate(&mut self, starting_point: Point) {
        if !self.mines.is_empty() {
            return;
        }
        while self.mines.len() < self.mine_count {
            let p = Point::new(
                random::<usize>() % self.width,
                random::<usize>() % self.height,
            );
            if p.dist(&starting_point) > 1 {
                self.mines.insert(p);
            }
        }
    }

    fn get_tile(&self, point: Point) -> Option<Tile> {
        if !self.is_valid_point(point) {
            return None;
        }
        if self.flags.contains(&point) {
            Some(Tile::Flag)
        } else if self.mines.contains(&point) {
            Some(Tile::Mine)
        } else {
            let count = point.neighbors().filter(|p| self.mines.contains(p)).count();
            if count == 0 {
                Some(Tile::Blank)
            } else {
                Some(Tile::Warn(count))
            }
        }
    }

    /**
     * Returns `true` if the game is still finished, false if the game ends
     */
    pub fn reveal(&mut self, point: Point) {
        if matches!(self.get_status(), GameStatus::Blank) {
            self.generate(point);
            self.game_status = GameStatus::InProgress;
        }

        if let Some(tile) = self.get_tile(point) {
            self.revealed.insert(point, tile);
            match tile {
                Tile::Mine => {
                    self.game_status = GameStatus::Lost;
                }
                Tile::Blank => {
                    let mut stack = vec![];
                    stack.extend(point.neighbors());
                    while let Some(p) = stack.pop() {
                        if self.revealed.contains_key(&p) {
                            continue;
                        }
                        if let Some(tile) = self.get_tile(p) {
                            self.revealed.insert(p, tile);
                            if let Tile::Blank = tile {
                                stack.extend(p.neighbors().filter(|p| self.is_valid_point(*p)));
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        if self.revealed().len() + self.flagged().len() == self.width * self.height {
            self.game_status = GameStatus::Completed;
        }
    }

    pub fn toggle_flag(&mut self, point: Point) -> bool {
        if self.revealed.contains_key(&point) {
            return false;
        }

        if self.flags.contains(&point) {
            self.flags.remove(&point);
        } else {
            self.flags.insert(point);
        }
        true
    }

    pub fn action(&mut self, action: GameAction) -> bool {
        match action {
            GameAction::Reveal(point) => {
                let revealed_before = self.revealed.len();
                self.reveal(point);
                let revealed_after = self.revealed.len();
                revealed_after > revealed_before
            }
            GameAction::ToggleFlag(point) => {
                self.toggle_flag(point);
                self.flagged().contains(&point)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameStatus {
    InProgress,
    Completed,
    Lost,
    Blank,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameAction {
    Reveal(Point),
    ToggleFlag(Point),
}

#[derive(Debug)]
pub enum NewMinesweeperError {
    InvalidWidth,
    InvalidHeight,
    TooManyMines { area: usize, mines: usize },
}

impl Display for NewMinesweeperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWidth => write!(f, "Invalid Width"),
            NewMinesweeperError::InvalidHeight => write!(f, "Invalid Height"),
            NewMinesweeperError::TooManyMines { area, mines } => {
                write!(f, "Too many mines ({mines}) for the given area ({area})")
            }
        }
    }
}

impl Error for NewMinesweeperError {}
