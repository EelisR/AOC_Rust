use std::collections::HashSet;
use std::collections::VecDeque;

pub enum MarkerLength {
    Part1(usize),
    Part2(usize),
}

pub fn solve(buffer: &String, marker_length: MarkerLength) {

    let marker_length = match marker_length {
        MarkerLength::Part1(length) => length,
        MarkerLength::Part2(length) => length,
    };

    let marker: &mut VecDeque<char> = &mut VecDeque::new();

    for (i, char) in buffer.chars().enumerate() {

        if marker.len() == marker_length {
            marker.pop_front();
        }

        marker.push_back(char);

        let mut set: HashSet<char> = HashSet::new();

        for c in marker.clone() {
            set.insert(c);
        }

        if set.len() == marker_length {
            print!("{}:  ", i + 1);
            for mark in marker.clone() {
                print!("{}", mark);
            }
            break;
        }
    }
}
