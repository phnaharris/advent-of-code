use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        // let item: Vec<&str> = split.collect();

        let action = String::from(split[0]);
        let count = split[1].parse::<i32>().unwrap();

        if action == *"forward" {
            horizontal += count;
        } else if action == *"down" {
            depth += count;
        } else {
            depth -= count;
        }
    }

    horizontal * depth
}

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        // let item: Vec<&str> = split.collect();

        let action = String::from(split[0]);
        let count = split[1].parse::<i32>().unwrap();

        if action == *"forward" {
            horizontal += count;
            depth += aim * count;
        } else if action == *"down" {
            aim += count;
        } else {
            aim -= count;
        }
    }

    horizontal * depth
}
