mod days;

use std::env;
use crate::days::day1::{
    question_one,
    question_two,
};

/// Programme principal
fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs");

    question_one(&path);
    question_two(&path);
}
