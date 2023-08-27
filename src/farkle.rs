use crate::models::{Farkle, FarkleSolution, SolutionType};
use rand::prelude::*;

impl Farkle {
    pub fn new_hand() -> Farkle {
        Farkle {
            max_dizes: 6,
            current_score: 0,
            keep_dize: vec![],
            roll_dize: vec![],
            best_solution: FarkleSolution::default(),
        }
    }

    pub fn roll_dize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut dizes: Vec<usize> = vec![];

        for _ in 0..self.max_dizes - self.keep_dize.len() {
            let number: usize = rng.gen_range(1..=6);
            dizes.push(number);
        }

        dizes.sort();

        self.best_solution = FarkleSolution::default();

        self.roll_dize = dizes;
    }

    pub fn keep(&mut self, index: usize) {
        let value = self
            .roll_dize
            .get(index)
            .expect("failed to get value from roll dizes on index");

        self.keep_dize.push(value.clone());
    }

    pub fn get_best_solution(&mut self) -> FarkleSolution {
        self.check_for_multiple_dizes();

        self.check_for_straight();

        self.check_for_full_house();

        self.check_for_singles();

        self.best_solution.clone()
    }
}
