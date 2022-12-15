mod part1;
mod part2;

use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufReader},
};

use part1::part1;
use part2::part2;

fn to_decimal(slice: Vec<u8>) -> i64 {
    let mut res: i64 = 0;

    (0..slice.len()).for_each(|i| {
        res += (slice[i] as i64) * i64::pow(2, (slice.len() - i - 1) as u32);
    });

    res
}

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    // let (gamma, epsilon) = part1(reader);
    // println!("{:?}", to_decimal(gamma));
    // println!("{:?}", to_decimal(epsilon));
    // println!(
    //     "Result: {:?}",
    //     to_decimal(gamma.clone()) * to_decimal(epsilon.clone())
    // );

    let (oxy, carbonic) = part2(reader);
    // println!("{:?}", to_decimal(oxy));
    // println!("{:?}", to_decimal(carbonic));
    println!("Result: {:?}", to_decimal(oxy) * to_decimal(carbonic));

    Ok(())
}
