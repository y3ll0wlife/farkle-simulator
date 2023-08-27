use std::ops::RangeInclusive;

use crate::models::{Farkle, FarkleSolution, SolutionType};
use rand::{prelude::*, seq::index};

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
            solution_type: SolutionType::None,
        };

        for i in 1..=self.max_dizes {
            for j in 1..=6 {
                if self.count_duplicate_of_value(i) == j {
                    let points = self.get_points_multiple(i, j);

                    if best_solution.points < points {
                        best_solution = FarkleSolution {
                            points,
                            keep_index: self.get_position_of_same_value(i),
                            solution_type: SolutionType::MultipleOfDizes,
                        }
                    }
                }
            }
        }

        {
            let mut points = 0;
            let mut indexes: Vec<usize> = vec![];
            for (index, dize_value) in self.roll_dize.iter().enumerate() {
                points += match dize_value {
                    1 => {
                        indexes.push(index);
                        100
                    }
                    5 => {
                        indexes.push(index);
                        50
                    }
                    _ => 0,
                }
            }

            if best_solution.points < points {
                best_solution = FarkleSolution {
                    points,
                    keep_index: indexes,
                    solution_type: SolutionType::Singles,
                }
            }
        }

        {
            self.roll_dize.sort();
            let indexes = (1..=self.max_dizes).collect::<Vec<usize>>();

            if self.roll_dize.iter().eq(indexes.iter()) {
                if best_solution.points < 1000 {
                    best_solution = FarkleSolution {
                        points: 1000,
                        keep_index: indexes.iter().map(|f| f - 1).collect(),
                        solution_type: SolutionType::Straight,
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
            1 | 2 => 0,
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

    fn get_position_of_same_value(&self, value: usize) -> Vec<usize> {
        let mut pos: Vec<usize> = vec![];

        for (index, dize) in self.roll_dize.iter().enumerate() {
            if &value == dize {
                pos.push(index);
            }
        }

        pos
    }
}
