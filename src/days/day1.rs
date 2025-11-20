use super::common::{Exercise, Ops};
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Lines {
    a: Vec<i32>,
    b: Vec<i32>,
}

impl Lines {
    fn new() -> Lines {
        Lines {
            a: Vec::new(),
            b: Vec::new(),
        }
    }
}

pub struct Day1 {}

impl Ops<Lines> for Day1 {
    fn get_input_file_path(base_dir: &PathBuf) -> PathBuf {
        base_dir.join("day1.txt")
    }

    fn read_lines(file_path: &str) -> Lines {
        let mut lines = Lines::new();

        for line in read_to_string(file_path).unwrap().lines() {
            let parts = line.split("   ").collect::<Vec<&str>>();

            lines.a.push(parts[0].parse::<i32>().unwrap_or(0));
            lines.b.push(parts[1].parse::<i32>().unwrap_or(0));
        }

        lines
    }
}

impl Exercise<Lines> for Day1 {
    fn new() -> Day1 {
        Day1 {}
    }

    fn question_one(&self, base_dir: &PathBuf) {
        let path = Day1::get_input_file_path(base_dir);
        let mut lines = Day1::read_lines(path.to_str().unwrap());

        lines.a.sort();
        lines.b.sort();

        let mut total = 0;

        for i in 0..lines.a.len() {
            total += (lines.a[i] - lines.b[i]).abs();
        }

        println!("Question 1 : {}", total);
    }

    fn question_two(&self, base_dir: &PathBuf) {
        let path = Day1::get_input_file_path(base_dir);
        let lines = Day1::read_lines(path.to_str().unwrap());

        let mut total = 0;

        for a in lines.a {
            total += a * lines.b.iter().filter(|&b| *b == a).count() as i32;
        }

        println!("Question 2 : {}", total);
    }
}
