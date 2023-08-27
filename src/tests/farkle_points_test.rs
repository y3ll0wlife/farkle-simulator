use crate::models::SolutionType;
use crate::models::{Farkle, FarkleSolution};

#[test]
fn multiple_dices() {
    let mut farkle = Farkle::new_hand();

    farkle.roll_dice = vec![1, 1, 1, 5, 6, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 1000,
            keep_index: vec![0, 1, 2],
            solution_type: SolutionType::MultipleOfdices
        }
    );

    farkle.roll_dice = vec![1, 1, 1, 1, 1, 1];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 8000,
            keep_index: vec![0, 1, 2, 3, 4, 5],
            solution_type: SolutionType::MultipleOfdices
        }
    );

    farkle.roll_dice = vec![1, 1, 1, 1, 6, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 2000,
            keep_index: vec![0, 1, 2, 3],
            solution_type: SolutionType::MultipleOfdices
        }
    );

    farkle.roll_dice = vec![3, 3, 3, 1, 6, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 300,
            keep_index: vec![0, 1, 2],
            solution_type: SolutionType::MultipleOfdices
        }
    );

    farkle.roll_dice = vec![3, 3, 3, 1, 6, 3];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 600,
            keep_index: vec![0, 1, 2, 5],
            solution_type: SolutionType::MultipleOfdices
        }
    );
}

#[test]
fn straight() {
    let mut farkle = Farkle::new_hand();

    farkle.roll_dice = vec![1, 2, 3, 4, 5, 6];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 1000,
            keep_index: vec![0, 1, 2, 3, 4, 5],
            solution_type: SolutionType::Straight
        }
    );

    farkle.roll_dice = vec![2, 1, 3, 4, 5, 6];
    farkle.roll_dice.sort();
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 1000,
            keep_index: vec![0, 1, 2, 3, 4, 5],
            solution_type: SolutionType::Straight
        }
    );
}

#[test]
fn full_house() {
    let mut farkle = Farkle::new_hand();

    farkle.roll_dice = vec![2, 2, 1, 5, 5, 5];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 1000,
            keep_index: vec![3, 4, 5, 0, 1],
            solution_type: SolutionType::FullHouse
        }
    );
}

#[test]
fn singles() {
    let mut farkle = Farkle::new_hand();

    farkle.roll_dice = vec![1, 5, 3, 6, 3, 1];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 250,
            keep_index: vec![0, 1, 5],
            solution_type: SolutionType::Singles
        }
    );

    farkle.roll_dice = vec![1, 5, 5, 6, 3, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 200,
            keep_index: vec![0, 1, 2],
            solution_type: SolutionType::Singles
        }
    );

    farkle.roll_dice = vec![6, 5, 4, 1, 1, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 250,
            keep_index: vec![1, 3, 4],
            solution_type: SolutionType::Singles
        }
    );

    farkle.roll_dice = vec![1, 5, 4, 5, 1, 2];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 300,
            keep_index: vec![0, 1, 3, 4],
            solution_type: SolutionType::Singles
        }
    );
}

#[test]
fn none() {
    let mut farkle = Farkle::new_hand();

    farkle.roll_dice = vec![2, 2, 3, 6, 3, 6];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 0,
            keep_index: vec![],
            solution_type: SolutionType::None
        }
    );

    farkle.roll_dice = vec![2, 3, 4, 6, 4, 6];
    farkle.best_solution = FarkleSolution::default();
    assert_eq!(
        farkle.get_best_solution(),
        FarkleSolution {
            points: 0,
            keep_index: vec![],
            solution_type: SolutionType::None
        }
    );
}
