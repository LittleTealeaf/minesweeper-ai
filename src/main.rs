use crate::{minesweeper::{MineSweeper, Point}, solver::Solver};

mod minesweeper;
mod solver;

fn main() {
    let mut ms = MineSweeper::new(200, 60, 2000).unwrap();
    let mut solver = Solver::new(ms);
    solver.display_delay = 0;
    solver.show_map_steps = false;

    solver.solve();

    println!("{}", solver.game);
}
