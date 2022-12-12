use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_value(c: char) -> i32 {
    let c_int: i32 = c as i32;

    let result = match c_int {
        97..=122 => c_int - 96,
        65..=90 => c_int - 38,
        _ => panic!("Non-valid c")
    };
    return result;
}

pub fn run() {
    let file = File::open("inputs/d03").expect("Failed to open file");
    let buff = BufReader::new(file);
    let mut score = 0;

    for line in buff.lines() {
        let row = line.expect("Failed to read line");
        let (half1, half2) = row.split_at(row.len()/2);
        let mut duplicates = Vec::new();

        for c in half1.chars() {
            if half2.contains(c) && !duplicates.contains(&c) {
                score += char_value(c);
                duplicates.push(c);
            }
        }
    }
    println!("Final score: {}", score);
}