use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

const ROW: i32 = 6;
const COL: i32 = 40;

#[derive(Debug)]
pub enum EInstruction {
    NOOP,
    ADDX,
}

impl FromStr for EInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addx" => Ok(Self::ADDX),
            "noop" => Ok(Self::NOOP),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Circuit {
    pub instruction: EInstruction,
    pub offset: Option<i32>,
}

impl FromStr for Circuit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split_whitespace().collect();

        Ok(Circuit {
            instruction: EInstruction::from_str(temp[0]).unwrap(),
            offset: if temp.len() > 1 {
                Some(temp[1].parse().unwrap())
            } else {
                None
            },
        })
    }
}

pub fn fill_circle(x: i32, circle: i32, crt: &mut [Vec<char>]) {
    // // sprite position == current_circle - 1; 3
    let circle_row = (circle - 1) / COL;
    let circle_col = (circle - 1) % COL;
    let sprite_pos = [x - 1, x, x + 1];

    if sprite_pos.contains(&circle_col) {
        crt[circle_row as usize][circle_col as usize] = '█';
    }
}

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut current_circle = 1;
    let mut x = 1;
    let mut result = 0;
    let mut interest_signal_strength_circle = vec![20, 60, 100, 140, 180, 220];

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let circuit = Circuit::from_str(&line).unwrap();

        if interest_signal_strength_circle.contains(&current_circle) {
            result += current_circle * x;
            interest_signal_strength_circle.remove(0);
        }

        match circuit.instruction {
            EInstruction::NOOP => {
                current_circle += 1;
            }
            EInstruction::ADDX => {
                if interest_signal_strength_circle.contains(&(current_circle + 1)) {
                    result += (current_circle + 1) * x;
                    interest_signal_strength_circle.remove(0);
                }

                current_circle += 2;
                x += circuit.offset.unwrap();
            }
        }
    }

    result
}

pub fn part2(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut current_circle = 1;
    let mut x = 1;
    let mut crt = vec![vec!['.'; COL as usize]; ROW as usize];

    for line in BufRead::lines(reader) {
        // start current_circle
        let line = line.unwrap();
        let circuit = Circuit::from_str(&line).unwrap();

        match circuit.instruction {
            EInstruction::NOOP => {
                fill_circle(x, current_circle, &mut crt);
                current_circle += 1;
            }
            EInstruction::ADDX => {
                fill_circle(x, current_circle, &mut crt);
                fill_circle(x, current_circle + 1, &mut crt);
                current_circle += 2;
                x += circuit.offset.unwrap();
            }
        }

        // end current_circle
    }

    for i in 0..ROW {
        for j in 0..COL {
            print!("{}", crt[i as usize][j as usize]);
        }

        println!("");
    }

    crt
}
