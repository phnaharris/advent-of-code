mod part1;
mod part2;

use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
    string::ParseError,
};

use part1::part1;
use part2::part2;

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

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    // let positions = part1(reader);
    // println!("{:?}", positions);

    let crt = part2(reader);
    // println!("{:?}", crt);

    Ok(())
}
