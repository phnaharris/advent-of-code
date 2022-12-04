use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use crate::Pair;

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut count_overlap = 0;

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        let p = Pair::from_str(&line).unwrap();

        if p.check_if_overlap() {
            count_overlap += 1;
        }
    }

    count_overlap
}
