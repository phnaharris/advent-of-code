mod part1;
mod part2;

use std::{
    fs::File,
    io::{self, BufReader},
};

use part1::part1;
use part2::part2;

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    // let total_prior = part1(reader);
    // println!("{:?}", total_prior);

    let total_prior = part2(reader);
    println!("{:?}", total_prior);

    Ok(())
}
