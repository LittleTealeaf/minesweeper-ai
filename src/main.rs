use crate::{minesweeper::MineSweeper, solver::Solver};

mod minesweeper;
mod solver;

fn main() {
    let width = 100;
    let height = 40;
    let area = width * height;
    let mines = area / 5;
    let ms = MineSweeper::new(width, height, mines).unwrap();
    let mut solver = Solver::new(ms);
    solver.display_delay = 0;
    solver.show_map_steps = false;

    solver.solve();

    println!("{}", solver.game);
}
