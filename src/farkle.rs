use crate::models::{Farkle, FarkleSolution, SolutionType};
use rand::prelude::*;

impl Farkle {
    pub fn new_hand() -> Farkle {
        Farkle {
            max_dices: 6,
            current_score: 0,
            keep_dice: vec![],
            roll_dice: vec![],
            best_solution: FarkleSolution::default(),
        }
    }

    pub fn roll_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let mut dices: Vec<usize> = vec![];

        for _ in 0..self.max_dices - self.keep_dice.len() {
            let number: usize = rng.gen_range(1..=6);
            dices.push(number);
        }

        dices.sort();

        self.best_solution = FarkleSolution::default();

        self.roll_dice = dices;
    }

    pub fn keep(&mut self, index: usize) {
        let value = self
            .roll_dice
            .get(index)
            .expect("failed to get value from roll dices on index");

        self.keep_dice.push(value.clone());
    }

    pub fn get_best_solution(&mut self) -> FarkleSolution {
        self.check_for_multiple_dices();

        self.check_for_straight();

        self.check_for_full_house();

        self.check_for_singles();

        self.best_solution.clone()
    }
}
