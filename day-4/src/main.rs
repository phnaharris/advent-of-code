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

struct BingoGame {
    caller: Vec<i64>,
    tables: Vec<BingoTable>,
}

struct BingoTable {
    table: Vec<Vec<i32>>,
}

impl FromStr for BingoTable {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok()
    }
}

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    let final_score = part1(reader);
    println!("{:?}", final_score);

    Ok(())
}
