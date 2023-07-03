use std::{env, fs};

pub mod day1;
pub mod day2;
pub mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Cannot read the file");
    
    day3::solve(&contents)
 
}

