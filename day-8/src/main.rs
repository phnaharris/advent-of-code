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

    let (total_size, tree_map) = part1(reader);
    println!("{:?}", total_size);

    let highest_scenic_score = part2(tree_map);
    println!("{:?}", highest_scenic_score);

    Ok(())
}
