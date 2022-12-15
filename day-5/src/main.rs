mod part1;
mod part2;

use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use part1::part1;
use part2::part2;

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

    for line in io::BufRead::lines(reader) {
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

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    let (mut stacks, actions) = input_reader(reader).unwrap();

    // let top_crates = part1(&mut stacks, actions);
    // println!("{:?}", top_crates);

    let top_crates = part2(&mut stacks, actions);
    println!("{:?}", top_crates);

    Ok(())
}
