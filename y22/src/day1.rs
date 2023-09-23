use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut temp = vec![0; 0];
    let mut max = -1;
    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        if line.is_empty() {
            let current_sum: i32 = temp.iter().sum();

            if current_sum > max {
                max = current_sum;
            }

            temp.clear();
        } else {
            let calories = line.parse().unwrap();
            temp.push(calories);
        }
    }

    max
}

fn make_position(value: i32, max_vec: &mut Vec<i32>) {
    for i in 0..max_vec.len() {
        if value > max_vec[i] {
            (*max_vec).insert(i, value);
            (*max_vec).pop();
            return;
        }
    }
}

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut temp = vec![0; 0];
    let mut max = vec![-1; 3];

    for line in BufRead::lines(reader) {
        let line = line.unwrap();

        if line.is_empty() {
            let current_sum: i32 = temp.iter().sum();

            make_position(current_sum, &mut max);

            temp.clear();
        } else {
            let calories = line.parse().unwrap();
            temp.push(calories);
        }
    }

    max.into_iter().sum()
}
