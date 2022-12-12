use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let file = File::open("inputs/d08").expect("Failed to open file");
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


    let mut visible_trees: u32 = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut visible: bool = false;
            if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
                visible = true;
            } else {

                // Row checks
                let l_max_opt = row[..j].iter().max();
                match l_max_opt {
                    Some(l_max) => {
                        if l_max < tree {
                            visible = true;
                        }
                    },
                    None => panic!("Max value not found"),
                }

                let r_max_opt = row[(j+1)..].iter().max();
                match r_max_opt {
                    Some(r_max) => {
                        if r_max < tree {
                            visible = true;
                        }
                    },
                    None => panic!("Max value not found"),
                }

                if !visible {
                    // Column checks
                    let mut top_max = 0;
                    let mut bottom_max = 0;
                    for (i2, dif_row) in matrix.iter().enumerate() {
                        if dif_row[j] > top_max && i2 < i {
                            top_max = dif_row[j];
                        } else if dif_row[j] > bottom_max && i2 > i {
                            bottom_max = dif_row[j];
                        }
                    }

                    if top_max < *tree || bottom_max < *tree {
                        visible = true;
                    }
                }
            }
            
            if visible {
                visible_trees += 1; 
            }
        }
    }

    println!("Visible: {}", visible_trees);

}