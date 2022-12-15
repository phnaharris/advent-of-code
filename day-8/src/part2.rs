use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut char_before_marker = 0;

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        for i in 0..line.len() {}
    }

    char_before_marker
}
