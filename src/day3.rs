use std::collections::{HashMap, HashSet};

pub fn solve(str: &String) {
    let lines = str.lines();
    let map = get_numeric_table();
    let mut sum = 0;
    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);
        let intersecting_chars = find_intersecting_chars(first, second);
        for character in intersecting_chars.chars() {
            sum += map.get(&character).unwrap();
        }
    }
    println!("Sum is {}", sum);
}

fn get_numeric_table() -> HashMap<char, u32> {
    let big_chars: Vec<char> = ('A'..='Z').collect();
    let big_numbers: Vec<u32> = (27..=52).collect();

    let small_chars: Vec<char> = ('a'..='z').collect();
    let small_numbers: Vec<u32> = (1..=26).collect();

    let chars: Vec<&char> = big_chars.iter().chain(small_chars.iter()).collect();
    let numbers: Vec<&u32> = big_numbers.iter().chain(small_numbers.iter()).collect();

    let mut map = HashMap::new();
    for (character, number) in chars.iter().zip(numbers.iter()) {
        map.insert(**character, **number);
    }

    return map;
}

fn find_intersecting_chars(first: &str, second: &str) -> String {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();

    return first_set.intersection(&second_set).collect();
}
