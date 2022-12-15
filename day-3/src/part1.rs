use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn part1(reader: BufReader<File>) -> (Vec<u8>, Vec<u8>) {
    let mut count_zero = vec![0; 0];
    let mut count_one = vec![0; 0];

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        if count_zero.is_empty() {
            count_zero = vec![0; line.len()];
            count_one = vec![0; line.len()];
        }

        for i in 0..line.len() {
            if line.chars().nth(i) == Some('0') {
                count_zero[i] += 1;
            } else {
                count_one[i] += 1;
            }
        }
    }

    let mut gamma = vec![0; 0];
    let mut epsilon = vec![0; 0];

    println!("count_one.len: {:?}", count_one.len());
    println!("count_zero.len: {:?}", count_zero.len());

    println!("count_one: {:?}", count_one);
    println!("count_zero: {:?}", count_zero);

    for i in 0..count_zero.len() {
        let (g, e) = if count_zero[i] > count_one[i] {
            (0, 1)
        } else {
            (1, 0)
        };

        println!("Push: {:?}", i);
        gamma.push(g);
        epsilon.push(e);
    }

    println!("gamma.len: {:?}", gamma.len());
    println!("epsilon.len: {:?}", epsilon.len());

    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);

    (gamma, epsilon)
}
