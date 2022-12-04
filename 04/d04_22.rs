use std::fs::File;
use std::io::{BufRead, BufReader, Error};


fn parse_bounds(bounds: &str) -> (i32, i32) {
    match bounds.split_once("-") {
        Some((l_str, u_str)) => {
            let lower: i32 = l_str.parse().unwrap();
            let upper: i32 = u_str.parse().unwrap();
            return (lower, upper);
        },
        None => {
            return (-1,-1);
        }
    }
}


fn main() -> Result<(), Error> {
    let file = File::open("./input")?;
    let buff = BufReader::new(file);
    let mut score = 0;

    for line in buff.lines() {
        let row = line?;
        match row.split_once(',') {
            Some((elf1, elf2)) => {
                let (l_elf1, u_elf1) = parse_bounds(&elf1);
                let (l_elf2, u_elf2) = parse_bounds(&elf2);

                // If lower or upper bounds match, we know that one
                // of the assigments is fully contained in the other
                if l_elf1 == l_elf2 || u_elf1 == u_elf2 {
                    score += 1;
                } else if l_elf1 < l_elf2 && u_elf1 > u_elf2 {
                    score += 1;
                } else if l_elf1 > l_elf2 && u_elf1 < u_elf2 {
                    score += 1;
                }
            },
            None => {
                println!("Expected two elves");
            }
        }
    }

    println!("Final score: {}", score);
    Ok(())
}