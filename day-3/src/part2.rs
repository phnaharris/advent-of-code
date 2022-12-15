use std::fs::File;
use std::io::{self, BufReader};

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

    res as u8
}

fn take_the_rest(pos: usize, data: &Vec<u8>, res: &mut Vec<u8>) {
    if res.len() != data.len() {
        (pos..data.len()).for_each(|i| res.push(data[i]));
    }
}

pub fn part2(reader: BufReader<File>) -> (Vec<u8>, Vec<u8>) {
    let mut data_most: Vec<Vec<u8>> = vec![vec![0; 0]; 0];
    let mut data_least: Vec<Vec<u8>> = vec![vec![0; 0]; 0];

    for line in io::BufRead::lines(reader) {
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
        println!("data_most.len: {:?}", data_most.len());
        println!("data_least.len: {:?}", data_least.len());

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

    println!("oxy: {:?}", oxy);
    println!("carbonic: {:?}", carbonic);

    (oxy, carbonic)
}
