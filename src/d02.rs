use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

fn round_win(l: RPSChoice, r: RPSChoice) -> i32 {
    // 2: l wins, 1: tie, 0: r wins
    if l == r {
        return 1;
    } else {
        let result = match l {
            RPSChoice::Rock if r == RPSChoice::Scissors => 2,
            RPSChoice::Paper if r == RPSChoice::Rock => 2,
            RPSChoice::Scissors if r == RPSChoice::Paper => 2,
            _ => 0,
        };
        return result;
    }
}

fn str_to_rps(c_str: &str) -> RPSChoice {
    let choice = match c_str {
        "A" | "X" => RPSChoice::Rock,
        "B" | "Y" => RPSChoice::Paper,
        "C" | "Z" => RPSChoice::Scissors,
        &_ => panic!("Non-valid choice"),
    };
    return choice;
}


fn calc_match(me: &str, opp: &str) -> i32 {
    let my_rps = str_to_rps(me);
    let opp_rps = str_to_rps(opp);

    let my_rps_value = match my_rps {
        RPSChoice::Rock => 1,
        RPSChoice::Paper => 2,
        RPSChoice::Scissors => 3
    };

    let win_score = 3 * round_win(my_rps, opp_rps);
    let result = my_rps_value + win_score;
    return result;
}

pub fn run() {
    let mut score = 0;

    let file = File::open("inputs/d02").expect("Failed to open file");
    let buff = BufReader::new(file);

    for line in buff.lines() {
        let row = line.expect("Failed to read line");
        let values: Vec<&str> = row.split(" ").collect();
        score += calc_match(values[1], values[0]);
    }

    println!("final score: {}", score);
}