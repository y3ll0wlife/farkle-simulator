use crate::models::Farkle;

mod farkle;
mod models;

#[cfg(test)]
mod tests;

fn main() {
    let mut farkle = Farkle::new_hand();
    farkle.roll_dize();

    println!("{:#?}", farkle);
    println!("{:#?}", farkle.get_best_solution());
}
