mod part1;
mod part2;

use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use part1::part1;
use part2::part2;

struct SectionInRange {
    start: i64,
    end: i64,
}

struct Pair {
    first: SectionInRange,
    second: SectionInRange,
}

impl FromStr for SectionInRange {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_start, _end) = s.split_once('-').unwrap();

        Ok(SectionInRange {
            start: _start.parse().unwrap(),
            end: _end.parse().unwrap(),
        })
    }
}

impl FromStr for Pair {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(',').unwrap();

        Ok(Pair {
            first: SectionInRange::from_str(first).unwrap(),
            second: SectionInRange::from_str(second).unwrap(),
        })
    }
}

impl Pair {
    fn check_if_fully_contain(&self) -> bool {
        if self.first.start <= self.second.start && self.first.end >= self.second.end {
            true
        } else {
            self.second.start <= self.first.start && self.second.end >= self.first.end
        }
    }

    fn check_if_overlap(&self) -> bool {
        !(self.first.end < self.second.start || self.second.end < self.first.start)
    }
}
fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    // let count_fully_contain = part1(reader);
    // println!("{:?}", count_fully_contain);

    let count_overlap = part2(reader);
    println!("{:?}", count_overlap);

    Ok(())
}
