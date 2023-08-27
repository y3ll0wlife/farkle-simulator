use crate::models::{Farkle, SolutionType};

mod farkle;
mod farkle_points;
mod models;
#[cfg(test)]
mod tests;

fn main() {
    let mut farkle = Farkle::new_hand();

    let mut i = 0;

    loop {
        farkle.roll_dice();
        i += 1;

        let solution = farkle.get_best_solution();

        match solution.solution_type {
            SolutionType::None => {
                println!("{:#?}", farkle);
                println!("{}", i);

                break;
            }
            _ => continue,
        };
    }
}
