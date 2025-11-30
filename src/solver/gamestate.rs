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

    pub fn add_flag(&'a mut self, point: Point) {
        self.flags.insert(point);
    }

    pub fn add_safe(&'a mut self, point: Point) {
        self.safe.insert(point);
    }

    pub fn flags(&'a self) -> impl Iterator<Item = &'a Point> {
        self.flags.iter().chain(self.game.flagged().iter())
    }

    pub fn revealed(&'a self) -> impl Iterator<Item = (Point, Tile)> {
        self.game
            .revealed()
            .clone()
            .into_iter()
            .chain(self.safe.iter().map(|point| (*point, Tile::Empty)))
    }

    pub fn is_revealed(&'a self, point: &Point) -> bool {
        self.safe.contains(point) || self.game.revealed().contains_key(point)
    }

    pub fn is_flag(&self, point: &Point) -> bool {
        self.flags.contains(point) || self.game.flagged().contains(point)
    }

    pub fn to_actions(&self) -> impl Iterator<Item = GameAction> {
        chain!(
            self.flags.iter().map(|p| GameAction::Flag(*p)),
            self.safe.iter().map(|p| GameAction::Reveal(*p))
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

    pub fn get_unknown_neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        point
            .neighbors()
            .filter(|p| !(self.is_revealed(p) || self.is_flag(p)))
    }
}
