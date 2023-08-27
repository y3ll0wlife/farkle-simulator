use crate::models::{Farkle, FarkleSolution, SolutionType};

impl Farkle {
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

    pub fn check_for_multiple_dizes(&mut self) {
        for i in 1..=self.max_dizes {
            for j in 1..=6 {
                if self.count_duplicate_of_value(i) == j {
                    let points = self.get_points_multiple(i, j);

                    if self.best_solution.points < points {
                        self.best_solution = FarkleSolution {
                            points,
                            keep_index: self.get_position_of_same_value(i),
                            solution_type: SolutionType::MultipleOfDizes,
                        }
                    }
                }
            }
        }
    }

    pub fn check_for_straight(&mut self) {
        let indexes = (1..=self.max_dizes).collect::<Vec<usize>>();

        if self.roll_dize.iter().eq(indexes.iter()) {
            if self.best_solution.points < 1000 {
                self.best_solution = FarkleSolution {
                    points: 1000,
                    keep_index: indexes.iter().map(|f| f - 1).collect(),
                    solution_type: SolutionType::Straight,
                }
            }
        }
    }

    pub fn check_for_full_house(&mut self) {
        let mut found_three_of_one = false;
        let mut indexes: Vec<usize> = vec![];

        for i in 1..=self.max_dizes {
            if self.count_duplicate_of_value(i) == 3 {
                found_three_of_one = true;
                indexes.append(&mut self.get_position_of_same_value(i));
            }
        }

        if !found_three_of_one {
            return;
        }

        let mut found_two_of_one = false;
        for i in 1..=self.max_dizes {
            if self.count_duplicate_of_value(i) == 2 {
                found_two_of_one = true;
                indexes.append(&mut self.get_position_of_same_value(i));
            }
        }

        if found_two_of_one && self.best_solution.points < 1000 {
            self.best_solution = FarkleSolution {
                points: 1000,
                keep_index: indexes,
                solution_type: SolutionType::FullHouse,
            }
        }
    }

    pub fn check_for_singles(&mut self) {
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

        if self.best_solution.points < points {
            self.best_solution = FarkleSolution {
                points,
                keep_index: indexes,
                solution_type: SolutionType::Singles,
            }
        }
    }
}
