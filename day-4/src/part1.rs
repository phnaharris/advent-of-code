use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use crate::Pair;

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut fully_contain = 0;

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        let p = Pair::from_str(&line).unwrap();

        if p.check_if_fully_contain() {
            fully_contain += 1;
        }
    }

    fully_contain
}
