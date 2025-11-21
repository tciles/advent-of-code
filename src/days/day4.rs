use super::common::{Exercise, Ops};
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Day4 {}

impl Ops<Vec<String>> for Day4 {
    fn get_input_file_path(base_dir: &PathBuf) -> PathBuf {
        base_dir.join("day4.txt")
    }

    fn read_lines(file_path: &str) -> Vec<String> {
        let mut lines = Vec::new();

        for line in read_to_string(file_path).unwrap().lines() {
            lines.push(line.to_string());
        }

        lines
    }
}

impl Exercise<Vec<String>> for Day4 {
    fn new() -> Day4 {
        Day4 {}
    }

    fn question_one(&self, base_dir: &PathBuf) {
        let path = Day4::get_input_file_path(base_dir);
        let lines = Day4::read_lines(path.to_str().unwrap());

        let mut total = 0;

        for i in 0..lines.len() {
            let line = lines.get(i).unwrap();
            let bytes = line.as_bytes();

            let top_idx = if (i as i32) - 1 < 0 { 0 } else { i - 1 };
            let bottom_idx = if i + 1 >= bytes.len() { i } else { i + 1 };

            let previous_bytes = lines[top_idx].as_bytes();
            let next_bytes = lines[bottom_idx].as_bytes();

            for j in 0..bytes.len() {
                let c = bytes[j] as char;

                if c != 'A' {
                    continue;
                }

                let mut horizontal = String::new();
                let mut vertical = String::new();
                let mut left_diag = String::new();
                let mut right_diag = String::new();

                let left_left_idx = if (j as i32) - 2 < 0 { 0 } else { j - 2 };
                let left_idx = if (j as i32) - 1 < 0 { 0 } else { j - 1 };
                let right_idx = if j + 1 >= bytes.len() { j } else { j + 1 };

                horizontal.push(bytes[left_left_idx] as char);
                horizontal.push(bytes[left_idx] as char);
                horizontal.push(bytes[j] as char);
                horizontal.push(bytes[right_idx] as char);

                vertical.push(previous_bytes[j] as char);
                vertical.push(bytes[j] as char);
                vertical.push(next_bytes[j] as char);

                left_diag.push(previous_bytes[left_idx] as char);
                left_diag.push(bytes[j] as char);
                left_diag.push(next_bytes[right_idx] as char);

                right_diag.push(previous_bytes[right_idx] as char);
                right_diag.push(bytes[j] as char);
                right_diag.push(next_bytes[left_idx] as char);

                if horizontal == "XSAM"
                    || horizontal == "XMAS"
                    || vertical == "XSAM"
                    || vertical == "XMAS"
                    || left_diag == "XSAM"
                    || left_diag == "XMAS"
                    || right_diag == "XSAM"
                    || right_diag == "XMAS"
                {
                    total += 1;
                }
            }
        }

        println!("Question 1 : {:?}", total);
    }

    fn question_two(&self, base_dir: &PathBuf) {
        let path = Day4::get_input_file_path(base_dir);
        let lines = Day4::read_lines(path.to_str().unwrap());

        println!("Question 1 : {:?}", 0);
    }
}
