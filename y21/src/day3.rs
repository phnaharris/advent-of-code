use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn to_decimal(slice: Vec<u8>) -> i64 {
    let mut res: i64 = 0;

    (0..slice.len()).for_each(|i| {
        res += (slice[i] as i64) * i64::pow(2, (slice.len() - i - 1) as u32);
    });

    res
}

pub fn part1(reader: BufReader<File>) -> i64 {
    let mut count_zero = vec![0; 0];
    let mut count_one = vec![0; 0];

    for line in BufRead::lines(reader) {
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

    for i in 0..count_zero.len() {
        let (g, e) = if count_zero[i] > count_one[i] {
            (0, 1)
        } else {
            (1, 0)
        };

        gamma.push(g);
        epsilon.push(e);
    }

    to_decimal(gamma) * to_decimal(epsilon)
}

fn find_common(most: bool, pos: usize, data: &mut Vec<Vec<u8>>) -> u8 {
    let mut count_one = 0;
    let mut count_zero = 0;

    (0..data.len()).for_each(|i| {
        (count_one, count_zero) = if data[i][pos] == 0 {
            (count_one, count_zero + 1)
        } else {
            (count_one + 1, count_zero)
        }
    });

    let res = if most {
        if count_zero > count_one {
            0
        } else {
            1
        }
    } else if count_zero > count_one {
        1
    } else {
        0
    };

    (*data) = (*data)
        .drain(..)
        .filter(|x| x[pos] == res)
        .collect::<Vec<Vec<u8>>>();

    res
}

fn take_the_rest(pos: usize, data: &Vec<u8>, res: &mut Vec<u8>) {
    if res.len() != data.len() {
        (pos..data.len()).for_each(|i| res.push(data[i]));
    }
}

pub fn part2(reader: BufReader<File>) -> i64 {
    let mut data_most: Vec<Vec<u8>> = vec![vec![0; 0]; 0];
    let mut data_least: Vec<Vec<u8>> = vec![vec![0; 0]; 0];

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let mut temp = vec![0; 0];

        for i in 0..line.len() {
            if line.chars().nth(i) == Some('0') {
                temp.push(0);
            } else {
                temp.push(1);
            }
        }

        data_most.push(temp.clone());
        data_least.push(temp.clone());
    }

    let mut oxy: Vec<u8> = vec![0; 0];
    let mut carbonic: Vec<u8> = vec![0; 0];

    for i in 0..data_most[0].len() {
        if data_most.len() == 1 {
            // take the rest
            take_the_rest(i, &data_most[0], &mut oxy);
        } else {
            let current_most = find_common(true, i, &mut data_most);
            oxy.push(current_most);
        }

        if data_least.len() == 1 {
            // take the rest
            take_the_rest(i, &data_least[0], &mut carbonic);
        } else {
            let current_least = find_common(false, i, &mut data_least);
            carbonic.push(current_least);
        }
    }

    to_decimal(oxy) * to_decimal(carbonic)
}
