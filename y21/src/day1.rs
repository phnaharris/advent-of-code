use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut count = 0;
    let mut last = 9999999;

    for line in BufRead::lines(reader) {
        let current = line.unwrap().parse::<i32>().unwrap();

        if current > last {
            count += 1;
        }

        last = current;
    }

    count
}

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut count = 0;
    let mut last;
    let continous_element = 3;
    let mut current_sum = vec![0; 0];

    for line in BufRead::lines(reader) {
        let current = line.unwrap().parse::<i32>().unwrap();

        if current_sum.len() < continous_element {
            current_sum.push(current);
            continue;
        } else {
            last = current_sum.iter().sum::<i32>();

            current_sum.remove(0);
            current_sum.push(current);
        }

        if current_sum.iter().sum::<i32>() > last {
            count += 1;
        }
    }

    count
}
