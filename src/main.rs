use crate::macros_calculator::{
    caloric_intake, caloric_treshold, macro_split, Activity, Diet, Gender, Goal, Person,
};

pub mod macros_calculator;

fn main() {
    let luca = Person::new(
        String::from("Luca"),
        33,
        Gender::Male,
        173,
        70,
        Activity::SuperActive,
    );

    println!("{}", luca);

    let caloric_treshold = caloric_treshold(luca);

    println!("Luca's caloric treshold is {}", caloric_treshold);

    let goal = Goal::WeightLoss;

    println!(
        "Luca's caloric intake should be {} for {}",
        caloric_intake(caloric_treshold, &goal),
        goal
    );

    let diet = Diet::LowCarb;
    let (carbs, protein, fat) =
        macro_split(caloric_intake(caloric_treshold, &goal), &diet).to_grams();

    println!("Considering a {} diet, the macros should be distributed as follows: carbs {}g, protein {}g, fat {}g", &diet, carbs, protein, fat);
}
