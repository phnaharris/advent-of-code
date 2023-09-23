use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const MAX_STACK: usize = 10;

#[derive(Clone, Debug)]
pub struct Stack {
    pub crates: Vec<char>,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Action {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = s.split_whitespace().collect();

        Ok(Action {
            count: line[1].parse().unwrap(),
            from: line[3].parse().unwrap(),
            to: line[5].parse().unwrap(),
        })
    }
}

impl Action {
    fn new() -> Self {
        Action {
            count: 0,
            from: 0,
            to: 0,
        }
    }
}

impl Stack {
    fn new(len: usize) -> Vec<Self> {
        vec![
            Stack {
                crates: vec!['A'; 0],
            };
            len
        ]
    }
}

fn input_reader(
    reader: BufReader<File>,
) -> Result<(Vec<Stack>, Vec<Action>), std::string::ParseError> {
    let mut stacks = Stack::new(MAX_STACK);
    let mut actions = vec![Action::new(); 0];

    let mut is_stack = true;

    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        if line.trim().is_empty() {
            is_stack = false;
            continue;
        }

        if is_stack {
            (0..line.len()).for_each(|i| {
                let current_char = line.chars().nth(i).unwrap();
                if current_char.is_ascii_uppercase() {
                    // col = i
                    // 012 3 456 7 8910 11 121314
                    // index i => col = 3i-2..3i+2
                    // ax + b | a(x+1) + b - ax - b = a = 4
                    // 4x + b => 4+b = 1 | 4.2+b = 5 => b = -3
                    // index = i => col = 4x - 3 = i
                    let stack_index = (i + 3) / 4;
                    stacks[stack_index].crates.insert(0, current_char);
                }
            })
        } else {
            actions.push(Action::from_str(line.as_str()).unwrap());
        };
    }

    Ok((stacks, actions))
}

pub fn part1(reader: BufReader<File>) -> String {
    let (mut stacks, actions) = input_reader(reader).unwrap();

    for action in actions {
        let count = action.count;
        let from = action.from;
        let to = action.to;

        for _i in 0..count {
            let temp = stacks[from].crates.pop().unwrap();
            stacks[to].crates.push(temp);
        }
    }

    let mut top_crates = String::from("");

    (1..stacks.len()).for_each(|i| {
        top_crates.push(*stacks[i].crates.last().unwrap());
    });

    top_crates
}

pub fn part2(reader: BufReader<File>) -> String {
    let (mut stacks, actions) = input_reader(reader).unwrap();

    for action in actions {
        let count = action.count;
        let from = action.from;
        let to = action.to;

        let length = stacks[from].crates.len();
        let temp: &mut [char] = &mut stacks[from]
            .crates
            .drain(length - count..)
            .collect::<Vec<char>>();

        stacks[to].crates.append(&mut Vec::from(temp));
    }

    let mut top_crates = String::from("");

    (1..stacks.len()).for_each(|i| {
        top_crates.push(*stacks[i].crates.last().unwrap());
    });

    top_crates
}
