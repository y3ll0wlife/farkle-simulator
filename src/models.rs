#[derive(Debug)]
pub struct Farkle {
    pub max_dizes: usize,
    pub keep_dize: Vec<usize>,
    pub roll_dize: Vec<usize>,
    pub current_score: usize,
}

#[derive(Debug)]
pub struct FarkleSolution {
    pub points: usize,
    pub keep_index: Vec<usize>,
    pub solution_type: SolutionType,
}

#[derive(Debug)]
pub enum SolutionType {
    None,
    MultipleOfDizes,
    Singles,
    Straight,
}
