use std::fs;
use std::env;
use std::collections::HashSet;

fn no_dup(vec: &Vec<char>) -> bool {
    let mut uniq = HashSet::new();
    return vec.iter().all(move |x| uniq.insert(x));
}


// First argument is the input file and the second is the width of the start-of-packet
// marker
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let unique_chars: usize = args[2].parse().expect("Invalid unique_chars argument"); 
    let datastream = fs::read_to_string(file_path).expect("Unable to read file");

    let mut char_vec = Vec::new();
    let mut found_flag = false;

    for (i, c) in datastream.chars().enumerate() {
        if char_vec.len() == unique_chars {
            if no_dup(&char_vec) {
                found_flag = true;
                println!("position: {}", i);
                break;
            }
            char_vec.remove(0);
        }
        char_vec.push(c);
    }
    if !found_flag {
        println!("Start-of-packet marker not found!");
    }
}