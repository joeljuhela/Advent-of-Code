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

    let lines: Vec<String> = buff.lines().collect::<Result<_, _>>().unwrap();

    for lines_chunk in lines.chunks(3) {
        let line1 = &lines_chunk[0];
        let mut duplicates = Vec::new();

        for c in line1.chars() {
            if !duplicates.contains(&c) {
                let mut found_in_all = true;
                for line in lines_chunk {
                    found_in_all = line.contains(c);
                    if !found_in_all {
                        break;
                    }
                }

                if found_in_all {
                    score += char_value(c);
                    duplicates.push(c);
                }
            }
        }
    }

    println!("Final score: {}", score);
}