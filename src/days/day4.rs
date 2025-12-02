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

        let xmas: Vec<u8> = vec![0x4D, 0x41, 0x53]; // MAS
        let samx: Vec<u8> = vec![0x53, 0x41, 0x4D, 0x58]; // SAMX

        let nb_lines = lines.len();

        for i in 0..nb_lines {
            let line = lines.get(i).unwrap();
            let bytes = line.as_bytes();
            let len = bytes.len();

            for j in 0..len {
                let c = bytes[j];

                if c != 0x58 { // X
                    continue;
                }

                // LEFT
                let x_l1 = if j >= 1 && j - 1 >= 0 { j - 1 } else { 0 };
                let x_l2 = if j >= 2 && j - 2 >= 0 { j - 2 } else { 0 };
                let x_l3 = if j >= 3 && j - 3 >= 0 { j - 3 } else { 0 };

                // RIGHT
                let x1 = if j + 1 < len { j + 1 } else { len - 1 };
                let x2 = if j + 2 < len { j + 2 } else { len - 1 };
                let x3 = if j + 3 < len { j + 3 } else { len - 1 };

                // TOP
                let y_t1 = if i >= 1 && i - 1 >= 0 { i - 1 } else { 0 };
                let y_t2 = if i >= 2 && i - 2 >= 0 { i - 2 } else { 0 };
                let y_t3 = if i >= 3 && i - 3 >= 0 { i - 3 } else { 0 };

                // BOTTOM
                let y1 = if i + 1 < nb_lines { i + 1 } else { nb_lines - 1 };
                let y2 = if i + 2 < nb_lines { i + 2 } else { nb_lines - 1 };
                let y3 = if i + 3 < nb_lines { i + 3 } else { nb_lines - 1 };

                // TOP LINES
                let line_t1 = lines.get(y_t1).unwrap().as_bytes();
                let line_t2 = lines.get(y_t2).unwrap().as_bytes();
                let line_t3 = lines.get(y_t3).unwrap().as_bytes();

                // BOTTOM LINES
                let line_b1 = lines.get(y1).unwrap().as_bytes();
                let line_b2 = lines.get(y2).unwrap().as_bytes();
                let line_b3 = lines.get(y3).unwrap().as_bytes();

                // right
                let right = vec![bytes[x1], bytes[x2], bytes[x3]];
                // left
                let left = vec![bytes[x_l1], bytes[x_l2], bytes[x_l3]];
                // bottom
                let bottom = vec![line_b1[j], line_b2[j], line_b3[j]];
                // top
                let top = vec![line_t1[j], line_t2[j], line_t3[j]];
                // top-left
                let top_right = vec![line_t1[x_l1], line_t2[x_l2], line_t3[x_l3]];
                // top-right
                let top_left = vec![line_t1[x1], line_t2[x2], line_t3[x3]];
                // bottom-left
                let bottom_right = vec![line_b1[x_l1], line_b2[x_l2], line_b3[x_l3]];
                // bottom-right
                let bottom_left = vec![line_b1[x1], line_b2[x2], line_b3[x3]];

                if right == xmas {
                    total += 1;
                }

                if left == xmas {
                    total += 1;
                }

                if bottom == xmas {
                    total += 1;
                }

                if top == xmas {
                    total += 1;
                }

                if top_left == xmas {
                    total += 1;
                }

                if top_right == xmas {
                    total += 1;
                }

                if bottom_right == xmas {
                    total += 1;
                }

                if bottom_left == xmas {
                    total += 1;
                }
            }
        }

        println!("Question 1 : {:?}", total); // 2573
    }

    fn question_two(&self, base_dir: &PathBuf) {
        let path = Day4::get_input_file_path(base_dir);
        let lines = Day4::read_lines(path.to_str().unwrap());

        println!("Question 1 : {:?}", 0); // 1850
    }
}
