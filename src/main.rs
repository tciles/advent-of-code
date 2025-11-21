#![allow(warnings)]

mod days;

use crate::days::{common::Exercise, day3::Day3};
use std::env;

/// Programme principal
fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs");

    let day = Day3::new();
    day.question_one(&path);
    day.question_two(&path);
}
