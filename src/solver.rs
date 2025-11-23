use std::{
    collections::{HashMap, HashSet},
    iter::{empty, once},
    thread,
    time::Duration,
};

use itertools::{Itertools, chain};

use crate::minesweeper::{GameAction, MineSweeper, Point, Tile};

pub struct Solver {
    pub game: MineSweeper,
    pub display_delay: u64,
    pub show_map_steps: bool,
}

impl Solver {
    pub fn new(game: MineSweeper) -> Self {
        Self {
            game,
            display_delay: 0,

            show_map_steps: true,
        }
    }

    pub fn game_action(&mut self, action: GameAction) -> bool {
        match &action {
            GameAction::Reveal(point) => println!("Revealing {point}"),
            GameAction::ToggleFlag(point) => println!("Flagging {point}"),
        }

        let res = self.game.action(action);
        if self.show_map_steps {
            println!("{}", self.game);
        }

        if self.display_delay > 0 {
            thread::sleep(Duration::from_millis(self.display_delay));
        }

        res
    }

    fn frontier(&self) -> impl Iterator<Item = (Point, usize)> {
        self.hints().filter_map(|(point, n)| {
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

    fn hints(&self) -> impl Iterator<Item = (Point, usize)> {
        self.game
            .revealed()
            .iter()
            .filter_map(|(point, tile)| match tile {
                Tile::Warn(n) => Some((*point, *n)),
                _ => None,
            })
    }

    fn get_unknown_neighbors<'a>(&self, point: &'a Point) -> impl Iterator<Item = Point> {
        point
            .neighbors()
            .filter(|p| !(self.game.revealed().contains_key(p) | self.game.flagged().contains(p)))
    }
}

impl Solver {
    pub fn solve(&mut self) {
        if matches!(
            self.game.get_status(),
            crate::minesweeper::GameStatus::Blank
        ) {
            self.game_action(GameAction::Reveal(Point::new(
                self.game.width() / 2,
                self.game.height() / 2,
            )));
        }

        'a: while self.game.is_in_progress() {
            let mut perf = false;
            for action in self.flag_known_points().collect_vec() {
                println!("Flag Known Points");
                perf |= self.game_action(action);

                if !self.game.is_in_progress() {
                    break 'a;
                }
            }

            for action in self.reveal_known_safes().collect_vec() {
                println!("Reveal Safe");
                perf |= self.game_action(action);
                if !self.game.is_in_progress() {
                    break 'a;
                }
            }

            if !perf {
                println!("No more moves, stopping");
                break;
            }
        }

        match self.game.get_status() {
            crate::minesweeper::GameStatus::Lost => println!("GAME LOST!!"),
            crate::minesweeper::GameStatus::Completed => println!("GAME WON!"),
            _ => {}
        }
    }

    fn flag_known_points(&self) -> impl Iterator<Item = GameAction> {
        self.frontier()
            .filter_map(|(point, count)| {
                let unknown_neighbors = self.get_unknown_neighbors(&point).collect::<Vec<_>>();
                if unknown_neighbors.len() == count {
                    Some(
                        unknown_neighbors
                            .into_iter()
                            .filter(|p| !self.game.flagged().contains(p))
                            .map(GameAction::ToggleFlag),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .unique()
    }

    fn reveal_known_safes(&self) -> impl Iterator<Item = GameAction> {
        self.hints()
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
