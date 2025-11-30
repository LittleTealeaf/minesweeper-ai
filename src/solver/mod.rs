pub mod gamestate;
mod solve_static;

use std::{thread, time::Duration};

use itertools::Itertools;

use crate::{
    minesweeper::{GameAction, GameStatus, MineSweeper, Tile},
    point::Point,
    solver::gamestate::SolveState,
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

            let mut solve_state = SolveState::from_game(&self.game);

            solve_state.static_solve();

            for action in solve_state.into_actions().collect_vec() {
                changed |= self.game_action(action);
            }

            if !changed {
                break;
            }
        }
    }
}
