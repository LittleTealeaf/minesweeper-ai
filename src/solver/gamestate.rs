use std::collections::HashSet;

use itertools::{Itertools, chain};

use crate::{
    minesweeper::{GameAction, MineSweeper, Tile},
    point::Point,
};

#[derive(Clone, Debug)]
pub struct SolveState<'a> {
    game: &'a MineSweeper,
    flags: HashSet<Point>,
    safe: HashSet<Point>,
}

impl<'a> SolveState<'a> {
    pub fn from_game(game: &'a MineSweeper) -> Self {
        Self {
            game,
            flags: HashSet::new(),
            safe: HashSet::new(),
        }
    }

    pub fn add_flags(&mut self, points: impl IntoIterator<Item = Point>) -> bool {
        let count = self.flags.len();
        self.flags.extend(points);
        count != self.flags.len()
    }

    pub fn add_flag(&mut self, point: Point) -> bool {
        self.flags.insert(point)
    }

    pub fn add_safe(&mut self, point: Point) -> bool {
        self.safe.insert(point)
    }

    pub fn add_safes(&mut self, safes: impl IntoIterator<Item = Point>) -> bool {
        let count = self.safe.len();
        self.safe.extend(safes);
        count != self.safe.len()
    }

    pub fn flags(&self) -> impl Iterator<Item = &Point> {
        self.flags.iter().chain(self.game.flagged().iter())
    }

    pub fn revealed(&self) -> impl Iterator<Item = (Point, Tile)> {
        self.game
            .revealed()
            .clone()
            .into_iter()
            .chain(self.safe.iter().map(|point| (*point, Tile::Empty)))
    }

    pub fn is_revealed(&self, point: &Point) -> bool {
        self.safe.contains(point) || self.game.revealed().contains_key(point)
    }

    pub fn is_flag(&self, point: &Point) -> bool {
        self.flags.contains(point) || self.game.flagged().contains(point)
    }

    pub fn is_valid(&self) -> bool {
        self.get_game_hints()
            .all(|(point, count)| count >= point.neighbors().filter(|p| self.is_flag(p)).count())
    }

    pub fn into_actions(self) -> impl Iterator<Item = GameAction> {
        chain!(
            self.flags.into_iter().map(GameAction::Flag),
            self.safe.into_iter().map(GameAction::Reveal)
        )
    }

    pub fn get_game_hints(&self) -> impl Iterator<Item = (Point, usize)> {
        self.game
            .revealed()
            .iter()
            .filter_map(|(point, tile)| match tile {
                Tile::Hint(n) => Some((*point, *n)),
                _ => None,
            })
    }

    pub fn get_frontier(&self) -> impl Iterator<Item = (Point, usize)> {
        self.get_game_hints().filter_map(|(point, n)| {
            let count = point.neighbors().filter(|n| self.is_flag(n)).count();
            if n > count {
                Some((point, n - count))
            } else {
                None
            }
        })
    }

    pub fn get_neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        point.neighbors().filter(|p| self.game.is_valid_point(p))
    }

    pub fn get_unknown_neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        self.get_neighbors(point)
            .filter(|p| !(self.is_revealed(p) || self.is_flag(p)))
    }
}
