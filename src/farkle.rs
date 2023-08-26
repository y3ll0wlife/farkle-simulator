use crate::models::{Farkle, FarkleSolution};
use rand::prelude::*;

impl Farkle {
    pub fn new_hand() -> Farkle {
        Farkle {
            max_dizes: 6,
            current_score: 0,
            keep_dize: vec![],
            roll_dize: vec![],
        }
    }

    pub fn roll_dize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut dizes: Vec<usize> = vec![];

        for _ in 0..self.max_dizes - self.keep_dize.len() {
            let number: usize = rng.gen_range(1..=6);
            dizes.push(number);
        }

        self.roll_dize = dizes;
    }

    pub fn keep(&mut self, index: usize) {
        let value = self
            .roll_dize
            .get(index)
            .expect("failed to get value from roll dizes on index");

        self.keep_dize.push(value.clone());
    }

    pub fn get_best_solution(&mut self) -> Option<FarkleSolution> {
        let mut best_solution = FarkleSolution {
            points: 0,
            keep_index: vec![],
        };

        for i in 1..=6 {
            for j in 1..=6 {
                if self.count_duplicate_of_value(i) == j {
                    let points = self.get_points_multiple(i, j);

                    if best_solution.points < points {
                        best_solution = FarkleSolution {
                            points,
                            keep_index: vec![],
                        }
                    }
                }
            }
        }

        if best_solution.points > 0 {
            return Some(best_solution);
        }

        None
    }

    fn get_points_multiple(&self, base_value: usize, amount_of_same: usize) -> usize {
        let value = match base_value {
            1 => 1000,
            _ => base_value * 100,
        };

        let multipler = match amount_of_same {
            1 => 0,
            2 => 0,
            3 => 1,
            4 => 2,
            5 => 4,
            6 => 8,
            _ => todo!("Illegal argument for 'amount_of_same'"),
        };

        value * multipler
    }

    fn count_duplicate_of_value(&self, value: usize) -> usize {
        self.roll_dize.iter().filter(|&n| *n == value).count()
    }
}
