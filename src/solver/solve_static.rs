use std::iter::empty;

use itertools::Itertools;

use crate::{point::Point, solver::gamestate::SolveState};

impl<'a> SolveState<'a> {
    pub fn static_solve(&mut self) {
        while self.add_flags(self.flag_known_points().collect_vec())
            || self.add_safes(self.reveal_known_safes().collect_vec())
        {}
    }

    fn flag_known_points(&self) -> impl Iterator<Item = Point> {
        self.get_frontier()
            .filter_map(|(point, count)| {
                let neighbors = self.get_unknown_neighbors(&point).collect_vec();

                if count == neighbors.len() {
                    Some(neighbors)
                } else {
                    None
                }
            })
            .flatten()
    }

    fn reveal_known_safes(&self) -> impl Iterator<Item = Point> {
        self.get_game_hints()
            .filter_map(|(point, count)| {
                let flagged_neighbors = point.neighbors().filter(|p| self.is_flag(p)).count();
                if count == flagged_neighbors {
                    Some(self.get_unknown_neighbors(&point).collect_vec())
                } else {
                    None
                }
            })
            .flatten()
    }

    /*
     * If one's neighbors is completely enveloped in another neighbors, then the mine(s) has to be
     * in one of those
     */
}
