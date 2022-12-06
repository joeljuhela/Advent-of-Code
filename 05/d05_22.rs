use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Error};

// We know that the crate columns are structured in fixed
// sized blocks, and the position of crate value in that block is fixed
// following leads to structure "[V] "
const CRATE_COLUMN_WIDTH: usize = 4;
const CRATE_POS: usize = 1;

fn init_crate_vec(row: &str) -> Vec<Vec<char>> {
    let vec_width: usize = (row.len() + CRATE_COLUMN_WIDTH - 1)/ CRATE_COLUMN_WIDTH;
    let mut crate_vec = vec![Vec::new(); vec_width];
    build_crates(row, &mut crate_vec);
    return crate_vec
}

fn build_crates(row: &str, crate_vec: &mut Vec<Vec<char>>) {
    for (i, c) in row.chars().enumerate() {
        if i % CRATE_COLUMN_WIDTH == CRATE_POS && c != ' ' {
            let vec_pos: usize = i / CRATE_COLUMN_WIDTH;
            crate_vec[vec_pos].insert(0, c);
        }
    }
}

fn handle_move(row: &str, crate_vec: &mut Vec<Vec<char>>) {
    let str_vec: Vec<&str> = row.split(" ").collect();
    let crate_count: i32 = str_vec[1].parse().unwrap();
    let from: usize = str_vec[3].parse::<usize>().unwrap() - 1;
    let to: usize = str_vec[5].parse::<usize>().unwrap() - 1;
    
    for _n in 0..crate_count {
        match crate_vec[from].pop() {
            Some(c) => {
                crate_vec[to].push(c);
            },
            None => break // pile is empty, no more crates to move
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let buff = BufReader::new(file);
    let mut lines_iter = buff.lines();

    let first_line = match lines_iter.next() {
        Some(x) => x,
        None => panic!("Empty input file!"),
    };

    let first_row: String = first_line.unwrap();
    let mut crate_vec: Vec<Vec<char>> = init_crate_vec(&first_row);

    // Assume that input file starts with the "crate diagram"
    for line in lines_iter {
        let row = line?;

        if !row.is_empty() {
            // If row does not start with 'move', the line is used for building
            // The initial crate piles
            if row.starts_with("move") {
                handle_move(&row, &mut crate_vec);
            } else if !row.starts_with(" 1 ") {
                build_crates(&row, &mut crate_vec);
            }
        }
    }

    let mut result = String::new();
    for mut pile in crate_vec {
        let pop_value = pile.pop();

        if let Some(c) = pop_value {
            result.push(c);
        }
    }

    println!("Result: {}", &result);

    Ok(())
}