struct Instruction {
    move_amount: usize,
    from: usize,
    to: usize,
}

pub enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

struct Stack {
    crates: Vec<char>,
}

struct StackSet {
    stacks: Vec<Stack>,
}

impl Stack {
    fn pop(&mut self) -> Option<char> {
        return self.crates.pop();
    }

    fn push(&mut self, char: char) {
        self.crates.push(char);
    }
}

pub fn solve(str: &String, crane: CraneType) {
    let lines = str.lines();

    let (stacks_strings, instructions) = create_init(lines);

    let mut stack_set = get_initial_stacks(stacks_strings);
    let instruction_set = get_instruction_set(instructions);

    move_stacks(instruction_set, &mut stack_set, crane);

    for stack in stack_set.stacks {
        print!("{}", stack.crates.last().unwrap());
    }
}

fn move_stacks(instruction_set: Vec<Instruction>, stack_set: &mut StackSet, crane: CraneType) {
    match crane {
        CraneType::CrateMover9000 => {
            for instruction in instruction_set {
                for _ in 0..instruction.move_amount {
                    let item = stack_set.stacks[instruction.from - 1]
                        .pop()
                        .expect("take crate from stack");
                    stack_set.stacks[instruction.to - 1].push(item);
                }
            }
        },
        CraneType::CrateMover9001 => {
            for instruction in instruction_set {
                let mut move_stack: Vec<char> = Vec::new();
                for _ in 0..instruction.move_amount {
                    let item = stack_set.stacks[instruction.from - 1]
                        .pop()
                        .expect("take crate from stack");
                    move_stack.push(item);
                }
                move_stack.reverse();
                stack_set.stacks[instruction.to - 1].crates.extend(move_stack);
            }
        }
    }
}

fn create_init(lines: std::str::Lines) -> (Vec<&str>, Vec<&str>) {
    let mut stacks_strings: Vec<&str> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();
    let mut passed = false;
    for line in lines {
        if line.is_empty() {
            passed = true;
            continue;
        }
        if !passed {
            stacks_strings.push(line);
            continue;
        } else {
            instructions.push(line);
        }
    }
    for stack in &stacks_strings {
        println!("{}", stack);
    }
    (stacks_strings, instructions)
}

fn get_instruction_set(instructions: Vec<&str>) -> Vec<Instruction> {
    let mut instruction_set: Vec<Instruction> = Vec::new();

    for instruction in instructions {
        let mut split_instruction = instruction.split(" ");
        let constructed_instruction = Instruction {
            move_amount: split_instruction.nth(1).unwrap().parse().unwrap(),
            from: split_instruction.nth(1).unwrap().parse().unwrap(),
            to: split_instruction.nth(1).unwrap().parse().unwrap(),
        };
        instruction_set.push(constructed_instruction);
    }
    return instruction_set;
}

fn get_initial_stacks(strings: Vec<&str>) -> StackSet {
    let mut stacks: Vec<Stack> = Vec::new();

    let index_row = strings.last().unwrap();
    for i in 0..index_row.len() {
        let char = index_row.chars().nth(i).unwrap();

        if char.is_ascii_digit() {
            let mut crates: Vec<char> = Vec::new();
            for j in 0..strings.len() - 1 {
                let char = strings[j].chars().nth(i).unwrap();
                if char != ' ' {
                    crates.push(char)
                }
            }
            crates.reverse();
            stacks.push(Stack { crates });
        }
    }
    for stack in &stacks {
        println!("{:?}", stack.crates);
    }
    return StackSet { stacks };
}
