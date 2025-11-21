use super::common::{Exercise, Ops};
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Day2 {}

impl Ops<Vec<Vec<i32>>> for Day2 {
    fn get_input_file_path(base_dir: &PathBuf) -> PathBuf {
        base_dir.join("day2.txt")
    }

    fn read_lines(file_path: &str) -> Vec<Vec<i32>> {
        let mut lines = Vec::new();

        for line in read_to_string(file_path).unwrap().lines() {
            let parts = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            lines.push(parts);
        }

        lines
    }
}

impl Exercise<Vec<Vec<i32>>> for Day2 {
    fn new() -> Day2 {
        Day2 {}
    }

    fn question_one(&self, base_dir: &PathBuf) {
        let path = Day2::get_input_file_path(base_dir);
        let lines = Day2::read_lines(path.to_str().unwrap());

        let mut total = 0;

        for i in 0..lines.len() {
            let line = &lines[i];
            let invalid = self.is_invalid(line);

            if invalid {
                continue;
            }

            total += 1;
        }

        println!("Question 1 : {:?}", total);
    }

    fn question_two(&self, base_dir: &PathBuf) {
        let path = Day2::get_input_file_path(base_dir);
        let lines = Day2::read_lines(path.to_str().unwrap());

        let mut total = 0;

        for i in 0..lines.len() {
            let line = &lines[i];
            let mut invalid = self.is_invalid(line);

            if invalid {
                for i in 0..line.len() {
                    let mut new_line = line.clone();
                    new_line.remove(i);

                    if !self.is_invalid(&new_line) {
                        invalid = false;
                        break;
                    }
                }
            }

            if invalid {
                continue;
            }

            total += 1;
        }

        println!("Question 2 : {:?}", total);
    }
}

impl Day2 {
    fn is_invalid(&self, line: &Vec<i32>) -> bool {
        let direction = if line[0] < line[1] { 1 } else { -1 } ;
        let mut last_number = line[0];
        let mut invalid = false;

        for i in 0..line.len() {
            let n = line[i];

            if i == 0 {
                continue;
            }

            if direction == 1 && last_number > n { // ASC et n-1 > n = false
                invalid = true;
            } else if direction == -1 && last_number < n { // DESC et n-1 < n = false
                invalid = true;
            }

            let delta = (last_number - n).abs();

            if delta < 1 || delta > 3 {
                invalid = true;
            }

            last_number = n;
        }

        invalid
    }
}
