use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut total_prior = 0;

    for line in BufRead::lines(reader) {
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

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut total_prior = 0;
    let mut temp_group = vec![String::from(""); 0];

    for line in BufRead::lines(reader) {
        if temp_group.len() < 2 {
            temp_group.push(line.unwrap().clone());
            continue;
        }

        temp_group.push(line.unwrap().clone());

        for c in temp_group[0].chars() {
            if temp_group[1].contains(c) && temp_group[2].contains(c) {
                if c.is_uppercase() {
                    total_prior += c as i32 - 'A' as i32 + 27;
                } else {
                    total_prior += c as i32 - 'a' as i32 + 1;
                }

                break;
            }
        }

        temp_group.clear();
    }

    total_prior
}
