use std::{env, fs};

pub mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Cannot read the file");
 
    day1::solve_day_one_second(&contents)
}

