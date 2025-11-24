use std::{thread, time::Duration};

use itertools::Itertools;

use crate::{
    minesweeper::{GameAction, GameStatus, MineSweeper, Tile},
    point::Point,
};

pub struct Solver {
    game: MineSweeper,
    step_delay: u64,
    show_map_steps: bool,
}

impl Solver {
    pub fn new(game: MineSweeper) -> Self {
        Self {
            game,
            step_delay: 0,
            show_map_steps: true,
        }
    }

    pub fn game(&self) -> &MineSweeper {
        &self.game
    }

    pub fn set_step_delay(&mut self, delay: u64) {
        self.step_delay = delay
    }

    pub fn set_show_map_steps(&mut self, show_map_steps: bool) {
        self.show_map_steps = show_map_steps
    }

    fn game_action(&mut self, action: GameAction) -> bool {
        println!("{action}");
        let res = self.game.perform_action(action);

        if self.show_map_steps {
            println!("{}", self.game);
        }

        if self.step_delay > 0 {
            thread::sleep(Duration::from_millis(self.step_delay));
        }

        res
    }
}

/** Helper Functions */
impl Solver {
    fn get_frontier(&self) -> impl Iterator<Item = (Point, usize)> {
        self.get_game_hints().filter_map(|(point, n)| {
            let count = point
                .neighbors()
                .filter(|n| self.game.flagged().contains(n))
                .count();
            if n > count {
                Some((point, n - count))
            } else {
                None
            }
        })
    }

    fn get_game_hints(&self) -> impl Iterator<Item = (Point, usize)> {
        self.game
            .revealed()
            .iter()
            .filter_map(|(point, tile)| match tile {
                Tile::Hint(n) => Some((*point, *n)),
                _ => None,
            })
    }

    fn get_unknown_neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        point
            .neighbors()
            .filter(|p| !(self.game.revealed().contains_key(p) || self.game.flagged().contains(p)))
    }
}

impl Solver {
    pub fn solve(&mut self) {
        if matches!(self.game.status(), crate::minesweeper::GameStatus::New) {
            self.game_action(GameAction::Reveal(Point::new(
                self.game.width() / 2,
                self.game.height() / 2,
            )));
        }

        while matches!(self.game.status(), GameStatus::InProgress) {
            let mut changed = false;
            println!("hi");

            for action in self.flag_known_points().collect_vec() {
                changed |= self.game_action(action);
            }

            for action in self.reveal_known_safes().collect_vec() {
                changed |= self.game_action(action);
            }

            if !changed {
                break;
            }
        }
    }
}

/** Definitive Solving Methods */
impl Solver {
    fn flag_known_points(&self) -> impl Iterator<Item = GameAction> {
        self.get_frontier()
            .filter_map(|(p, count)| {
                // check the number of flags to the count
                if count
                    == p.neighbors()
                        .filter(|n| self.game.flagged().contains(n))
                        .count()
                {
                    Some(
                        p.neighbors()
                            .collect_vec()
                            .into_iter()
                            .filter(|l| {
                                !(self.game.flagged().contains(l)
                                    || self.game.revealed().contains_key(l))
                            })
                            .map(GameAction::Reveal),
                    )
                } else {
                    None
                }
            })
            .flatten()
    }

    fn reveal_known_safes(&self) -> impl Iterator<Item = GameAction> {
        self.get_game_hints()
            .filter_map(|(p, count)| {
                // check the number of flags to the count
                if count
                    == p.neighbors()
                        .filter(|n| self.game.flagged().contains(n))
                        .count()
                {
                    Some(
                        p.neighbors()
                            .collect_vec()
                            .into_iter()
                            .filter(|l| {
                                !(self.game.flagged().contains(l)
                                    || self.game.revealed().contains_key(l))
                            })
                            .map(GameAction::Reveal),
                    )
                } else {
                    None
                }
            })
            .flatten()
    }
}
