use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut temp = vec![0; 0];
    let mut max = -1;
    for line in io::BufRead::lines(reader) {
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
