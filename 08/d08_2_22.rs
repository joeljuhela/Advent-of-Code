use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to open file");
    let buff = BufReader::new(file);
    
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    // Build matrix
    for line in buff.lines() {
        let row = line.expect("Failed to read line");
        let mut row_vec: Vec<u32> = Vec::new();

        for c in row.chars() {
            let num_opt = c.to_digit(10);
            if let Some(num) = num_opt {
                row_vec.push(num);
            }
        }
        matrix.push(row_vec);
    }
    let height = matrix.len();
    let width = matrix[0].len();


    let mut highest_scenic_score: u32 = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {

            // if we are at the edge, the score will be zero, so we can just continue
            if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
                continue;
            } 

            // Row checks
            let mut lv: u32 = 0;
            for t in &row[..j] {
                if t < tree {
                    lv += 1;
                } else {
                    lv = 1;
                }
            }

            let mut rv: u32 = 0;
            for t in &row[(j+1)..] {
                if t < tree {
                    rv += 1;
                } else {
                    rv += 1;
                    break;
                }
            }

            // col checks
            let mut tv: u32 = 0;
            let mut bv: u32 = 0;

            for dif_row in &matrix[..i] {
                if dif_row[j] < *tree {
                    tv += 1;
                } else {
                    tv = 1;
                }
            }

            for dif_row in &matrix[(i+1)..] {
                if dif_row[j] < *tree {
                    bv += 1;
                } else {
                    bv += 1;
                    break;
                }
            }

            if j == 2 && i == 3 {
                println!("[{}][{}] {}, lv: {}, rv: {}, tv: {}, bv: {}", j,i,tree,lv,rv,tv,bv);
            }

            let scenic_score:u32 = lv*rv*tv*bv;

            if highest_scenic_score < scenic_score {
                highest_scenic_score = scenic_score;
            }
            
            
        }
    }

    println!("Highest scenic score: {}", highest_scenic_score);

}