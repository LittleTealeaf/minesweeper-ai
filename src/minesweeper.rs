use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

use rand::random;

use crate::point::Point;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Tile {
    Empty,
    Mine,
    Hint(usize),
    Flag,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "_"),
            Tile::Mine => write!(f, "M"),
            Tile::Hint(i) => write!(f, "{i}"),
            Tile::Flag => write!(f, "F"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameStatus {
    New,
    InProgress,
    Win,
    Loss,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameAction {
    Reveal(Point),
    Flag(Point),
    Unflag(Point),
    ToggleFlag(Point),
}

impl Display for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameAction::Reveal(point) => write!(f, "Reveal {point}"),
            GameAction::Flag(point) => write!(f, "Flag {point}"),
            GameAction::Unflag(point) => write!(f, "Unflag {point}"),
            GameAction::ToggleFlag(point) => write!(f, "Toggle Flag {point}"),
        }
    }
}

#[derive(Debug)]
pub struct MineSweeper {
    width: usize,
    height: usize,
    mine_count: usize,
    status: GameStatus,
    mines: HashSet<Point>,
    flags: HashSet<Point>,
    revealed: HashMap<Point, Tile>,
}

impl Display for MineSweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x, y);
                if let Some(tile) = self.revealed.get(&p) {
                    write!(f, "{tile}")?;
                } else if self.flags.contains(&p) {
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
    pub fn new(width: usize, height: usize, mines: usize) -> Result<Self, NewMinesweeperError> {
        if width == 0 {
            return Err(NewMinesweeperError::InvalidWidth);
        } else if height == 0 {
            return Err(NewMinesweeperError::InvalidHeight);
        } else if (height * width) < mines {
            return Err(NewMinesweeperError::TooManyMines {
                area: height * width,
                mines,
            });
        }

        Ok(Self {
            height,
            width,
            status: GameStatus::New,
            mine_count: mines,
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

    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        Point::iter_range(0, self.width(), 0, self.height())
    }

    pub fn revealed(&self) -> &HashMap<Point, Tile> {
        &self.revealed
    }

    pub fn flagged(&self) -> &HashSet<Point> {
        &self.flags
    }

    pub fn flag_point(&mut self, point: Point) -> bool {
        self.flags.insert(point)
    }

    pub fn unflag_point(&mut self, point: &Point) -> bool {
        self.flags.remove(point)
    }

    fn generate_mines(&mut self, starting_point: &Point) {
        while self.mines.len() < self.mine_count {
            let p = Point::new(
                random::<usize>() % self.width,
                random::<usize>() % self.height,
            );
            if p.dist_max(starting_point) > 1 {
                self.mines.insert(p);
            }
        }
    }

    fn get_tile(&self, point: &Point) -> Tile {
        if self.flags.contains(point) {
            Tile::Flag
        } else if self.mines.contains(point) {
            Tile::Mine
        } else {
            let count = point.neighbors().filter(|p| self.mines.contains(p)).count();
            if count == 0 {
                Tile::Empty
            } else {
                Tile::Hint(count)
            }
        }
    }

    pub fn reveal(&mut self, point: Point) -> bool {
        if matches!(self.status, GameStatus::New) {
            self.generate_mines(&point);
            self.status = GameStatus::InProgress
        }

        let mut changes = false;

        let mut stack = vec![point];
        while let Some(p) = stack.pop() {
            if self.revealed.contains_key(&p) {
                continue;
            }

            let tile = self.get_tile(&p);
            match &tile {
                Tile::Mine => self.status = GameStatus::Loss,
                Tile::Empty => {
                    stack.extend(p.neighbors());
                }
                _ => {}
            }
            changes = true;
            self.revealed.insert(p, tile);
        }

        changes
    }

    pub fn perform_action(&mut self, action: GameAction) -> bool {
        match self.status {
            GameStatus::Win | GameStatus::Loss => {
                return false;
            }
            _ => {}
        }
        match action {
            GameAction::Reveal(point) => self.reveal(point),
            GameAction::Flag(point) => self.flag_point(point),
            GameAction::Unflag(point) => self.unflag_point(&point),
            GameAction::ToggleFlag(point) => self.unflag_point(&point) || self.flag_point(point),
        }
    }
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
