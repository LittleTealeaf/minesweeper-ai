use std::{
    collections::{HashMap, HashSet},
    iter::empty,
};

use itertools::Itertools;

use crate::{
    minesweeper::{GameAction, MineSweeper, Tile},
    point::Point,
    solver::Solver,
};

struct GameState<'a> {
    width: usize,
    height: usize,
    flags: HashSet<Point>,
    last_flagged: Option<Point>,
    revealed: &'a HashMap<Point, Tile>,
}

impl<'a> GameState<'a> {
    fn from_game(game: &'a MineSweeper) -> Self {
        Self {
            height: game.height(),
            width: game.width(),
            flags: game.flagged().clone(),
            last_flagged: None,
            revealed: game.revealed(),
        }
    }

    fn with_flagged(&self, flag: Point) -> Self {
        let mut flags = self.flags.clone();
        flags.insert(flag);
        Self {
            height: self.height,
            width: self.width,
            last_flagged: Some(flag),
            flags,
            revealed: self.revealed,
        }
    }

    fn get_hints(&self) -> impl Iterator<Item = (Point, usize)> {
        self.revealed.iter().filter_map(|(point, tile)| match tile {
            Tile::Hint(i) => Some((*point, *i)),
            _ => None,
        })
    }

    fn check_valid_hints(&self) -> bool {
        self.get_hints().all(|(point, count)| {
            point.neighbors().filter(|p| self.flags.contains(p)).count() <= count
        })
    }
}

impl Solver {
    pub fn solve_with_depth(&self, depth: usize) -> impl Iterator<Item = GameAction> {
        let base_state = GameState::from_game(self.game());
        let mut frontiers = self
            .get_frontier()
            .flat_map(|(point, _)| {
                self.get_unknown_neighbors(&point)
                    .map(|p| base_state.with_flagged(p))
                    .collect_vec()
            })
            .collect_vec();

        empty()
    }
}
