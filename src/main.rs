mod days;

use days::{common::Exercise, day1::Day1};
use std::env;

/// Programme principal
fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs");

    let day1 = Day1::new();
    day1.question_one(&path);
    day1.question_two(&path);
}
