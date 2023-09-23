use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
pub fn part1(reader: BufReader<File>) -> i32 {
    let mut fully_contain = 0;

    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        let p = Pair::from_str(&line).unwrap();

        if p.check_if_fully_contain() {
            fully_contain += 1;
        }
    }

    fully_contain
}

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut count_overlap = 0;

    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        let p = Pair::from_str(&line).unwrap();

        if p.check_if_overlap() {
            count_overlap += 1;
        }
    }

    count_overlap
}
