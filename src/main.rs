use crate::{minesweeper::MineSweeper, solver::Solver};

mod minesweeper;
mod point;
mod solver;

fn main() {
    let width = 100;
    let height = 40;
    let area = width * height;
    let mines = area / 6;
    let ms = MineSweeper::new(width, height, mines).unwrap();
    let mut solver = Solver::new(ms);
    solver.set_step_delay(0);
    solver.set_show_map_steps(false);
    //
    solver.solve();
    //
    println!("{}", solver.game());
}
