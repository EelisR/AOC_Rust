#[derive(Debug)]
struct File {
    size: u32,
}

impl File {
    fn new(size: u32) -> Self {
        File { size }
    }
}

#[derive(Debug)]
struct Dir {
    size: u32,
    name: String,
    children: Vec<Output>,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            size: 0,
            name,
            children: vec![],
        }
    }
}

impl Default for Dir {
    fn default() -> Self {
        Dir {
            size: 0,
            name: String::new(),
            children: vec![],
        }
    }
}

#[derive(Debug)]
enum Input {
    CdInto { dir_name: String },
    CdOut,
    Ls,
}

#[derive(Debug)]
enum Output {
    File(File),
    Dir(Dir),
}

enum TerminalLine {
    Input(Input),
    Output(Output),
}

const MAX_DIR_SIZE: u32 = 100000;
const TOTAL_DISK_SPACE: u32 = 70000000;
const SPACE_NEEDED_FOR_UPDATE: u32 = 30000000;

pub fn solve(str: &str, second: bool) {
    let lines = str.lines();
    let terminal_lines: Vec<TerminalLine> = lines.map(parse_line).collect();

    let mut totals: Vec<u32> = Vec::new();
    let mut stack: Vec<Dir> = Vec::new();

    for line in terminal_lines {
        match line {
            TerminalLine::Input(input) => match input {
                Input::CdInto { dir_name } => stack.push(Dir::new(dir_name)),
                Input::CdOut => {
                    let total = stack.pop().unwrap().size;
                    totals.push(total);

                    if let Some(dir) = stack.last_mut() {
                        dir.size += total;
                    }
                }
                Input::Ls => continue,
            },
            TerminalLine::Output(output) => match output {
                Output::Dir(_) => continue,
                Output::File(file) => {
                    stack.last_mut().unwrap().size += file.size;
                }
            },
        }
    }

    while let Some(dir) = stack.pop() {
        let parent = stack.last_mut();
        match parent {
            Some(parent) => parent.size += dir.size,
            None => totals.push(dir.size),
        }
    }

    if second {
        let current_available_space = TOTAL_DISK_SPACE - totals.last().unwrap();
        println!("Current available space: {}", current_available_space);
        let mut possible_to_delete = totals
            .iter()
            .filter(|&&x| x + current_available_space > SPACE_NEEDED_FOR_UPDATE)
            .collect::<Vec<&u32>>();
        possible_to_delete.sort();

        print!("To delete: {}", possible_to_delete.first().unwrap());
    } else {
        let total = totals.iter().filter(|&x| x < &MAX_DIR_SIZE).sum::<u32>();
        print!("Total size: {}", total)
    }
}

fn parse_line(line: &str) -> TerminalLine {
    let mut words = line.split_whitespace();

    let first_word = words.next().unwrap();
    match first_word {
        "$" => parse_user_command(words),
        _ => parse_terminal_output(first_word, words),
    }
}

fn parse_terminal_output(first_word: &str, mut words: std::str::SplitWhitespace) -> TerminalLine {
    match first_word {
        "dir" => TerminalLine::Output(Output::Dir(Dir::new(words.next().unwrap().to_string()))),
        _ => {
            let size: Result<u32, std::num::ParseIntError> = first_word.parse();
            match size {
                Ok(size) => TerminalLine::Output(Output::File(File::new(size))),
                Err(_) => panic!("Unknown output {}", first_word),
            }
        }
    }
}

fn parse_user_command(mut words: std::str::SplitWhitespace) -> TerminalLine {
    let command = words.next().unwrap();
    match command {
        "cd" => parse_cd_command(words),
        "ls" => TerminalLine::Input(Input::Ls),
        _ => panic!("Unknown command {}", command),
    }
}

fn parse_cd_command(mut words: std::str::SplitWhitespace) -> TerminalLine {
    let dir = words.next().unwrap();
    match dir {
        ".." => TerminalLine::Input(Input::CdOut),
        _ => TerminalLine::Input(Input::CdInto {
            dir_name: dir.to_string(),
        }),
    }
}
