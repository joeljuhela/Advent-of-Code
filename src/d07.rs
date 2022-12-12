use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::iter::FromIterator;

const TOTAL_DISKSPACE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

pub fn run() {
    let file = File::open("inputs/d07").expect("Failed to open file");
    let buff = BufReader::new(file);

    let mut dir_file_content = HashMap::new();
    let mut cur_path: Vec<String> = Vec::new();

    for line in buff.lines() {
        let row = line.expect("Failed to read line");
        let command: Vec<&str> = row.split_whitespace().collect();

        match command[..] {
            ["$", "cd", ".."] => {
                cur_path.pop();
            },
            ["$", "cd", "/"] => {
                cur_path.clear();
                cur_path.push("/".to_string());
            },
            ["$", "cd", dir_name] => {
                cur_path.push(dir_name.to_string());
            },
            ["$", "ls"] => {},
            ["dir", _name] => {},
            [size, _file] => {
                let file_size: u32 = size.parse().expect("Couldn't parse line");
                for idx in 0..cur_path.len() {
                    let path = PathBuf::from_iter(&cur_path[..=idx]);
                    *dir_file_content.entry(path).or_insert(0) += file_size;
                }
            },
            _ => {}
        }
 
    }

    let mut sum_sizes: u32 = 0;
    let file_system_size: u32 = dir_file_content[&PathBuf::from("/")];
    let space_needed = UPDATE_SIZE - (TOTAL_DISKSPACE - file_system_size);
    let mut delete_size: u32 = 0;

    for (_dir, size) in dir_file_content {
        if size <= 100000 {
            sum_sizes += size;
        }
        if size >= space_needed {
            if delete_size == 0 {
                delete_size = size;
            } else if size < delete_size {
                delete_size = size;
            }
        }
    }

    println!("Sum of total sizes: {}, size of deleted dir: {}", sum_sizes, delete_size);
}