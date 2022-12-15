use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut final_score = 0;

    let reader = io::BufRead::lines(reader);

    let caller = reader.borrow().next();

    final_score
}
