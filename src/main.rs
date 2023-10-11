use crate::macros_calculator::{Activity, Gender, Person};

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
}
