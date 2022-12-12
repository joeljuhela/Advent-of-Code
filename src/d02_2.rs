use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn value(&self) -> i32 {
        let result = match &self {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3
        };
        return result;
    }
}

enum Outcome {
    Win,
    Tie,
    Lose,
}

fn win(rps: &RPSChoice) -> i32 {
    let result = match &rps {
        RPSChoice::Rock => RPSChoice::Paper.value(),
        RPSChoice::Paper => RPSChoice::Scissors.value(),
        RPSChoice::Scissors => RPSChoice::Rock.value(),
    };
    return result;
}

fn lose(rps: &RPSChoice) -> i32 {
    let result = match &rps {
        RPSChoice::Rock => RPSChoice::Scissors.value(),
        RPSChoice::Paper => RPSChoice::Rock.value(),
        RPSChoice::Scissors => RPSChoice::Paper.value(),
    };
    return result;
}

fn calc_rps_value(opp_rps: &RPSChoice, outcome: &Outcome) -> i32 {
    let result = match outcome {
        Outcome::Lose => lose(opp_rps),
        Outcome::Tie => opp_rps.value(),
        Outcome::Win => win(opp_rps),
    };
    return result;
}

fn str_to_outcome(o_str: &str) -> Outcome {
    let result = match o_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        &_ => panic!("Non-valid outcome"),
    };
    return result;
}

fn str_to_rps(c_str: &str) -> RPSChoice {
    let choice = match c_str {
        "A" => RPSChoice::Rock,
        "B" => RPSChoice::Paper,
        "C" => RPSChoice::Scissors,
        &_ => panic!("Non-valid choice"),
    };
    return choice;
}


fn calc_match(me: &str, opp: &str) -> i32 {
    let opp_rps = str_to_rps(opp);
    let my_outcome = str_to_outcome(me);

    let my_rps_value = calc_rps_value(&opp_rps, &my_outcome);
    let win_score = match my_outcome {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Lose => 0,
    };

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