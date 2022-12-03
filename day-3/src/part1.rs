use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut total_prior = 0;

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        let p1: String = line.chars().skip(0).take(line.len() / 2).collect();
        let p2: String = line
            .chars()
            .skip(line.len() / 2)
            .take(line.len() / 2)
            .collect();

        for c in p1.chars() {
            if p2.contains(c) {
                if c.is_uppercase() {
                    total_prior += c as i32 - 'A' as i32 + 27;
                } else {
                    total_prior += c as i32 - 'a' as i32 + 1;
                }
                break;
            }
        }
    }

    total_prior
}
