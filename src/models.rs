#[derive(Debug)]
pub struct Farkle {
    pub max_dizes: usize,
    pub keep_dize: Vec<usize>,
    pub roll_dize: Vec<usize>,
    pub current_score: usize,
    pub best_solution: FarkleSolution,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum SolutionType {
    None,
    MultipleOfDizes,
    Singles,
    Straight,
    FullHouse,
}
