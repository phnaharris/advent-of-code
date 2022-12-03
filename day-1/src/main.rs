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

    // let max = part1(reader);
    // println!("{:?}", max);

    let max = part2(reader);
    println!("{:?}", max);

    Ok(())
}
