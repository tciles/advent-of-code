use super::common::{Exercise, Ops};
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Day3 {}

impl Ops<Vec<String>> for Day3 {
    fn get_input_file_path(base_dir: &PathBuf) -> PathBuf {
        base_dir.join("day3.txt")
    }

    fn read_lines(file_path: &str) -> Vec<String> {
        let mut lines = Vec::new();

        for line in read_to_string(file_path).unwrap().lines() {
            lines.push(line.to_string());
        }

        lines
    }
}

impl Exercise<Vec<String>> for Day3 {
    fn new() -> Day3 {
        Day3 {}
    }

    fn question_one(&self, base_dir: &PathBuf) {
        let path = Day3::get_input_file_path(base_dir);
        let lines = Day3::read_lines(path.to_str().unwrap());

        let total = self.calc_total(lines, false);

        println!("Question 1 : {:?}", total);
    }

    fn question_two(&self, base_dir: &PathBuf) {
        let path = Day3::get_input_file_path(base_dir);
        let lines = Day3::read_lines(path.to_str().unwrap());

        let total = self.calc_total(lines, true);

        println!("Question 2 : {:?}", total);
    }
}

impl Day3 {
    fn calc_total(&self, lines: Vec<String>, with_do_dont: bool) -> i32 {
        let mut total = 0;
        let mut enabled = true;

        for line in lines {
            let bytes = line.as_bytes();

            for mut i in 0..bytes.len() {
                if true == with_do_dont {
                    if !enabled && i + 4 < bytes.len() && &line[i..i + 4] == "do()" {
                        enabled = true;
                        i += 4;

                        continue;
                    }

                    if enabled && i + 7 < bytes.len() && &line[i..i + 7] == "don't()" {
                        enabled = false;
                        i += 7;

                        continue;
                    }

                    if !enabled {
                        continue;
                    }
                }

                // Multiplication
                if i + 4 < bytes.len() && &line[i..i + 4] == "mul(" {
                    i += 4; // "("

                    let mut left = String::new();
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        left.push(bytes[i] as char);
                        i += 1;
                    }

                    i += 1; // virgule

                    let mut right = String::new();
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        right.push(bytes[i] as char);
                        i += 1;
                    }

                    if i > bytes.len() {
                        continue;
                    }

                    left = left.trim().to_string();
                    right = right.trim().to_string();

                    if left.is_empty() || right.is_empty() {
                        continue;
                    }

                    if bytes[i] as char == ')' {
                        let a = left.parse::<i32>().unwrap();
                        let b = right.parse::<i32>().unwrap();

                        total += a * b;
                    }
                }
            }
        }

        total
    }
}
