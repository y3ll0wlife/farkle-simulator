#[derive(Debug, PartialEq)]
pub struct Farkle {
    pub max_dices: usize,
    pub keep_dice: Vec<usize>,
    pub roll_dice: Vec<usize>,
    pub current_score: usize,
    pub best_solution: FarkleSolution,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FarkleSolution {
    pub points: usize,
    pub keep_index: Vec<usize>,
    pub solution_type: SolutionType,
}

impl FarkleSolution {
    pub fn default() -> FarkleSolution {
        FarkleSolution {
            points: 0,
            keep_index: vec![],
            solution_type: SolutionType::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolutionType {
    None,
    MultipleOfdices,
    Singles,
    Straight,
    FullHouse,
}
