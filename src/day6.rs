use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(buffer: &String) {
    let marker: &mut VecDeque<char> = &mut VecDeque::new();

    for (i, char) in buffer.chars().enumerate() {

        if marker.len() == 4 {
            marker.pop_front();
        }

        marker.push_back(char);

        let mut set: HashSet<char> = HashSet::new();

        for c in marker.clone() {
            set.insert(c);
        }

        if set.len() == 4 {
            print!("{}:  ", i + 1);
            for mark in marker.clone() {
                print!("{}", mark);
            }
            break;
        }
    }
}
