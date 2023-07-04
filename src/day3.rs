use std::collections::{HashMap, HashSet};

pub fn solve(str: &String) {
    let lines = str.lines();
    let map = get_numeric_table();
    let mut sum = 0;
    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);
        let intersecting_chars = find_intersecting_chars_multiple(vec![&first, &second]);

        for character in intersecting_chars.chars() {
            if let Some(number) = map.get(&character) {
                sum += number;
            }
        }
    }
    println!("Sum is {}", sum);
}

pub fn solve_second(str: &String) {
    let lines: Vec<&str> = str.lines().collect();
    let map = get_numeric_table();
    let groups: Vec<Vec<&str>> = lines.chunks(3).map(|chunk| chunk.to_vec()).collect();
    let mut sum = 0;

    for group in groups {
        let char_intersect = find_intersecting_chars_multiple(group);
        for character in char_intersect.chars() {
            if let Some(number) = map.get(&character) {
                sum += number;
            }
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

fn find_intersecting_chars_multiple(strings: Vec<&str>) -> String {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    for string in &strings {
        sets.push(string.chars().collect());
    }

    let mut set_to_manage: HashSet<char> = sets.pop().unwrap();
    let mut intersection: String = String::with_capacity(strings.len());

    for set in sets {
        intersection = set_to_manage.intersection(&set).collect();
        set_to_manage = intersection.chars().collect();
    }

    return intersection;
}
