use std::{
    fs::File,
    io::{self, BufReader},
    vec,
};

pub fn is_visible(r: &usize, c: &usize, tree_map: &Vec<Vec<usize>>) -> bool {
    let mut result1 = true;
    let mut result2 = true;
    let mut result3 = true;
    let mut result4 = true;

    for mr in 1..*r - 1 {
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result1 = false;
            break;
        }
    }

    for mr in *r + 1..tree_map.len() - 1 {
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result2 = false;
            break;
        }
    }

    for mc in 1..*c - 1 {
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result3 = false;
            break;
        }
    }

    for mc in *c + 1..tree_map[0].len() - 1 {
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result4 = false;
            break;
        }
    }

    result1 || result2 || result3 || result4
}

pub fn part1(reader: BufReader<File>) -> usize {
    let mut tree_map = vec![vec![0; 0]; 0];

    for line in io::BufRead::lines(reader) {
        let mut line = line.unwrap();

        let mut temp_vec = vec![0; 0];

        loop {
            let c = line.pop();
            match c {
                Some(n) => temp_vec.insert(0, n.to_digit(10).expect("not a valid number") as usize),
                None => break,
            }
        }

        tree_map.push(temp_vec);
    }

    let mut visibles = (tree_map.len() + tree_map[0].len()) * 2 - 4;

    for r in 1..tree_map.len() - 1 {
        for c in 1..tree_map[r].len() - 1 {
            if is_visible(&r, &c, &tree_map) {
                visibles += 1;
            }
        }
    }

    visibles
}
