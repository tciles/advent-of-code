#![allow(warnings)]

use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

/// Programme principal
fn main() {
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("inputs").join("2025").join("day2.txt");

    answer_one(&path);
    answer_two(&path);
}

fn answer_one(path: &PathBuf) {
    let lines = get_lines(&path);

    let mut total: u64 = 0;

    for line in lines {
        let a = line[0];
        let b = line[1];

        for i in a..(b + 1) {
            let c = i.to_string();

            if c.len() % 2 != 0 {
                continue;
            }

            let left_part = c[0..c.len() / 2].to_string();
            let right_part = c[c.len() / 2..].to_string();

            if left_part == right_part {
                total += i;
            }
        }
    }

    println!("Answer 1: {}", total);
}

fn answer_two(path: &PathBuf) {
    let lines = get_lines(&path);

    let mut total: u64 = 0;

    for line in lines {
        let a = line[0];
        let b = line[1];

        for i in a..(b + 1) {
            let c = i.to_string();

            for j in 0..c.len() {
                let p = &c[0..j];

                if c.replace(p, "").is_empty() {
                    total += i;
                    break;
                }
            }
        }
    }

    println!("Answer 2: {}", total);
}

fn get_lines(path: &PathBuf) -> Vec<Vec<u64>> {
    let mut lines = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        lines = line.trim().split(",")
            .map(|x| {
                x.trim().split("-")
                    .map(|s| {
                        s.parse::<u64>().unwrap()
                    })
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
    }

    lines
}
