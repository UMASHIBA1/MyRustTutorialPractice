extern crate phrases;

// use phrases::english::farewells;
// use phrases::english::greetings;

use phrases::english::{farewells, greetings};
use phrases::japanese;

fn main() {
    // println!("Hello in English: {}", phrases::english::greetings::hello());
    // println!(
    //     "Goodbye in English: {}",
    //     phrases::english::farewells::goodbye()
    // );

    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
