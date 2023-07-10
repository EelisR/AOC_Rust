struct Forest {
    trees: Vec<Tree>,
}

impl Forest {
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

type Height = usize;

#[derive(Clone)]
struct Tree {
    x: usize,
    y: usize,
    height: Height,
}

pub fn solve(string: &str) {
    let grid = parse_grid(string);
    solve_part1(grid);
}

fn solve_part1(forest: Forest) {
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
