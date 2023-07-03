use std::collections::HashMap;

enum RPS {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

fn create_score_map() -> HashMap<char, RPS> {
    let mut scores = HashMap::new();
    scores.insert('A', RPS::Rock(1));
    scores.insert('B', RPS::Paper(2));
    scores.insert('C', RPS::Scissors(3));
    scores.insert('X', RPS::Rock(1));
    scores.insert('Y', RPS::Paper(2));
    scores.insert('Z', RPS::Scissors(3));
    return scores;
}

enum RPSResult {
    Win,
    Lose,
    Draw,
}
fn create_result_map() -> HashMap<char, RPSResult> {
    let mut scores = HashMap::new();
    scores.insert('X', RPSResult::Lose);
    scores.insert('Y', RPSResult::Draw);
    scores.insert('Z', RPSResult::Win);
    return scores;
}

fn get_chars_from_line(line: &str) -> (char, char) {
    let mut chars = line.split_whitespace();
    let first = chars.next().unwrap().chars().next().unwrap();
    let second = chars.next().unwrap().chars().next().unwrap();

    return (first, second);
}

pub fn solve(str: &String) {
    let lines = str.lines();
    let scores = create_score_map();
    let mut total_score = 0;

    for line in lines {
        let (opponent, own) = get_chars_from_line(line);
        let opponent_hand = scores.get(&opponent).unwrap();
        let own_hand = scores.get(&own).unwrap();

        match own_hand {
            RPS::Rock(score) => match opponent_hand {
                RPS::Rock(_) => total_score += 3 + score,
                RPS::Paper(_) => total_score += score,
                RPS::Scissors(_) => total_score += 6 + score,
            },
            RPS::Paper(score) => match opponent_hand {
                RPS::Rock(_) => total_score += 6 + score,
                RPS::Paper(_) => total_score += 3 + score,
                RPS::Scissors(_) => total_score += score,
            },
            RPS::Scissors(score) => match opponent_hand {
                RPS::Rock(_) => total_score += score,
                RPS::Paper(_) => total_score += 6 + score,
                RPS::Scissors(_) => total_score += 3 + score,
            },
        }
    }

    print!("Total score: {}", total_score);
}

pub fn solve_second(str: &String) {
    let lines = str.lines();
    let scores = create_score_map();
    let results = create_result_map();
    let mut total_score = 0;

    for line in lines {
        let (opponent, own) = get_chars_from_line(line);
        let opponent_hand = scores.get(&opponent).unwrap();
        let wanted_result = results.get(&own).unwrap();
        match wanted_result {
            RPSResult::Win => match opponent_hand {
                RPS::Rock(_) => total_score += 6 + 2,
                RPS::Paper(_) => total_score += 6 + 3,
                RPS::Scissors(_) => total_score += 6 + 1,
            },
            RPSResult::Lose => match opponent_hand {
                RPS::Rock(_) => total_score += 3,
                RPS::Paper(_) => total_score += 1,
                RPS::Scissors(_) => total_score += 2,
            },
            RPSResult::Draw => match opponent_hand {
                RPS::Rock(score) => total_score += 3 + score,
                RPS::Paper(score) => total_score += 3 + score,
                RPS::Scissors(score) => total_score += 3 + score,
            },
        }
    }

    print!("Total score: {}", total_score);
}
