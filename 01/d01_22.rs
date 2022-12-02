use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    

    let mut top3: [i32; 3] = [0; 3];
    let mut cur_cals: i32 = 0;
    
    for line in reader.lines() {
        let row = line?;

        if row.is_empty() {
            let mut rep = cur_cals;

            for val in &mut top3 {
                if rep > *val {
                    let temp = *val;
                    *val = rep;
                    rep = temp;
                }
            }
        
            cur_cals = 0;
        } else {
            let cals: i32 = row.parse::<i32>().unwrap();
            cur_cals += cals;
        }
    }

    println!("Total calories of the top3 are: {}", top3.iter().sum::<i32>());

    Ok(())
}