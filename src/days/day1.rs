use std::fs::read_to_string;
use std::path::{PathBuf};

struct Lines {
    a: Vec<i32>,
    b: Vec<i32>
}

impl Lines {
    fn new() -> Lines {
        Lines {
            a: Vec::new(),
            b: Vec::new()
        }
    }
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

pub fn question_one(base_dir: &PathBuf) {
    let full_path = base_dir.join("day1.txt");

    if !full_path.exists() {
        panic!("Day1.txt not found");
    }

    let mut lines = read_lines(full_path.to_str().unwrap());
    lines.a.sort();
    lines.b.sort();

    println!("Question 1 : {}", calc_total(&lines));
}

pub fn question_two(base_dir: &PathBuf) {
    let full_path = base_dir.join("day1.txt");

    if !full_path.exists() {
        panic!("Day1.txt not found");
    }

    let lines = read_lines(full_path.to_str().unwrap());

    println!("Question 2 : {}", calc_total_v2(&lines));
}

fn calc_total(lines: &Lines) -> i32 {
    let mut total = 0;

    for i in 0..lines.a.len() {
        let delta = (lines.a[i] - lines.b[i]).abs();

        total += delta;
    }

    total
}

fn calc_total_v2(lines: &Lines) -> i32 {
    let mut total = 0;

    for i in 0..lines.a.len() {
        let a = lines.a[i];
        let count_in_list_b = lines.b.iter().filter(|&b| *b == a).count() as i32;
        let delta = a * count_in_list_b;

        total += delta;
    }

    total
}
