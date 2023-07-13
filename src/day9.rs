use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn solve(str: &str) {
    let directions = parse_directions(str);
    solve_part_1(directions);
}

fn solve_part_1(directions: Vec<Direction>) {
    let mut tail = Point { x: 0, y: 0 };
    let mut head = Point { x: 0, y: 0 };

    let mut visited_points: HashSet<Point> = HashSet::new();

    for dir in directions {
        match dir {
            Direction::U(d) => {
                for _ in 0..d {
                    head.y += 1;
                    if head.y > tail.y + 1 {
                        tail.y += 1;
                        if head.x != tail.x {
                            tail.x = head.x;
                        }
                        visited_points.insert(tail);
                    }
                }
            }
            Direction::D(d) => {
                for _ in 0..d {
                    head.y -= 1;
                    if head.y < tail.y - 1 {
                        tail.y -= 1;
                        if head.x != tail.x {
                            tail.x = head.x;
                        }
                        visited_points.insert(tail);
                    }
                }
            }
            Direction::L(d) => {
                for _ in 0..d {
                    head.x -= 1;
                    if head.x < tail.x - 1 {
                        tail.x -= 1;
                        if head.y != tail.y {
                            tail.y = head.y;
                        }
                        visited_points.insert(tail);
                    }
                }
            }
            Direction::R(d) => {
                for _ in 0..d {
                    head.x += 1;
                    if head.x > tail.x + 1 {
                        tail.x += 1;
                        if head.y != tail.y {
                            tail.y = head.y;
                        }
                        visited_points.insert(tail);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", visited_points.len());
}

fn parse_directions(str: &str) -> Vec<Direction> {
    let lines = str.lines();

    let mut directions = Vec::new();
    for line in lines {
        let mut split = line.split(" ");
        let direction = split
            .next()
            .expect("the first word should be one of U, D, L, R");
        let distance = split.next().expect("the second word should exist");
        let distance = distance
            .parse::<usize>()
            .expect("the second word should be a number");

        let direction = match direction {
            "U" => Direction::U(distance),
            "D" => Direction::D(distance),
            "L" => Direction::L(distance),
            "R" => Direction::R(distance),
            _ => panic!("Unknown direction"),
        };

        directions.push(direction);
    }

    directions
}
