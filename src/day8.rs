struct Forest {
    trees: Vec<Tree>,
}

impl Forest {
    fn scenic_score(&self, current: &Tree) -> usize {
        let mut trees_up: Vec<&Tree> = self
            .trees
            .iter()
            .filter(|tree| tree.y < current.y && tree.x == current.x)
            .collect();

        let visible_trees_up = get_tree_count(&mut trees_up, current, Axel::Up);

        let mut trees_down: Vec<&Tree> = self
            .trees
            .iter()
            .filter(|tree| tree.y > current.y && tree.x == current.x)
            .collect();

        let visible_trees_down = get_tree_count(&mut trees_down, current, Axel::Down);

        let mut trees_right: Vec<&Tree> = self
            .trees
            .iter()
            .filter(|tree| tree.x > current.x && tree.y == current.y)
            .collect();

        let visible_trees_right = get_tree_count(&mut trees_right, current, Axel::Right);

        let mut trees_left: Vec<&Tree> = self
            .trees
            .iter()
            .filter(|tree| tree.x < current.x && tree.y == current.y)
            .collect();

        let visible_trees_left = get_tree_count(&mut trees_left, current, Axel::Left);

        return visible_trees_up * visible_trees_down * visible_trees_right * visible_trees_left;
    }

    fn is_tree_visible(&self, current: &Tree) -> bool {
        let trees_up = self
            .trees
            .iter()
            .filter(|tree| tree.y < current.y && tree.x == current.x)
            .max_by_key(|tree| tree.height);

        match trees_up {
            Some(trees_up) => {
                if trees_up.height < current.height {
                    return true;
                }
            }
            None => {
                return true;
            }
        }

        let trees_down = self
            .trees
            .iter()
            .filter(|tree| tree.y > current.y && tree.x == current.x)
            .max_by_key(|tree| tree.height);

        match trees_down {
            Some(trees_up) => {
                if trees_up.height < current.height {
                    return true;
                }
            }
            None => {
                return true;
            }
        }

        let trees_right = self
            .trees
            .iter()
            .filter(|tree| tree.x > current.x && tree.y == current.y)
            .max_by_key(|tree| tree.height);

        match trees_right {
            Some(trees_up) => {
                if trees_up.height < current.height {
                    return true;
                }
            }
            None => {
                return true;
            }
        }

        let trees_left = self
            .trees
            .iter()
            .filter(|tree| tree.x < current.x && tree.y == current.y)
            .max_by_key(|tree| tree.height);

        match trees_left {
            Some(trees_up) => {
                if trees_up.height < current.height {
                    return true;
                }
            }
            None => {
                return true;
            }
        }

        return false;
    }
}

enum Axel {
    Up,
    Down,
    Left,
    Right,
}

fn get_tree_count(trees: &mut Vec<&Tree>, current: &Tree, axel: Axel) -> usize {
    match axel {
        Axel::Down => trees.sort_by_key(|tree| tree.y),
        Axel::Up => {
            trees.sort_by_key(|tree| tree.y);
            trees.reverse();
        },
        Axel::Left => {
            trees.sort_by_key(|tree| tree.x);
            trees.reverse();
        }
        Axel::Right => {
            trees.sort_by_key(|tree| tree.x);
        }
    }

    if trees.len() == 0 {
        return 0;
    }

    let mut visible_count_up = 0;
    for tree in trees {
        visible_count_up += 1;
        if tree.height >= current.height {
            return visible_count_up as usize;
        }
    }

    return visible_count_up as usize;
}

type Height = usize;

#[derive(Clone)]
struct Tree {
    x: usize,
    y: usize,
    height: Height,
}

pub fn solve(string: &str) {
    let forest = parse_grid(string);
    solve_part1(&forest);
    solve_part2(&forest);
}

fn solve_part2(forest: &Forest) {
    let mut highest_scenic_score = 0;
    for tree in &forest.trees {
        let scenic_score = forest.scenic_score(&tree);
        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }
    println!("Highest scenic score: {highest_scenic_score}");
}

fn solve_part1(forest: &Forest) {
    let mut visible_count = 0;
    for tree in &forest.trees {
        if forest.is_tree_visible(&tree) {
            visible_count += 1;
        }
    }

    println!("Visible tree count: {visible_count}")
}

fn parse_grid(string: &str) -> Forest {
    let lines = string.lines();

    let mut grid = Forest { trees: Vec::new() };
    for (y, line) in lines.enumerate() {
        for (x, tree) in line.chars().enumerate() {
            let height = tree.to_digit(10).unwrap() as Height;
            let cell = Tree { height, x, y };
            grid.trees.push(cell);
        }
    }

    grid
}
