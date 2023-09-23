use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn is_visible_1(r: &usize, c: &usize, tree_map: &Vec<Vec<usize>>) -> bool {
    let mut result1 = true;
    let mut result2 = true;
    let mut result3 = true;
    let mut result4 = true;

    for mr in 0..*r {
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result1 = false;
            break;
        }
    }

    for mr in *r + 1..tree_map.len() {
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result2 = false;
            break;
        }
    }

    for mc in 0..*c {
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result3 = false;
            break;
        }
    }

    for mc in *c + 1..tree_map[0].len() {
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result4 = false;
            break;
        }
    }

    result1 || result2 || result3 || result4
}

pub fn multiply(temp: &mut usize, temp_in_round: usize) {
    *temp *= if temp_in_round == 0 { 1 } else { temp_in_round }
}
pub fn is_visible_2(
    r: &usize,
    c: &usize,
    tree_map: &[Vec<usize>],
    visibles: &mut [Vec<usize>],
) -> bool {
    let mut result1 = true;
    let mut result2 = true;
    let mut result3 = true;
    let mut result4 = true;

    let mut temp_in_round = 0;
    let mut temp = 1;

    for mr in (0..*r).rev() {
        // up
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result1 = false;
            temp_in_round += 1;
            break;
        } else {
            temp_in_round += 1;
        }
    }
    multiply(&mut temp, temp_in_round);
    temp_in_round = 0;

    for mr in *r + 1..tree_map.len() {
        // down
        if tree_map[*r][*c] <= tree_map[mr][*c] {
            result2 = false;
            temp_in_round += 1;

            break;
        } else {
            temp_in_round += 1;
        }
    }
    multiply(&mut temp, temp_in_round);
    temp_in_round = 0;

    for mc in (0..*c).rev() {
        // left
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result3 = false;
            temp_in_round += 1;
            break;
        } else {
            temp_in_round += 1;
        }
    }
    multiply(&mut temp, temp_in_round);
    temp_in_round = 0;

    for mc in *c + 1..tree_map[0].len() {
        // right
        if tree_map[*r][*c] <= tree_map[*r][mc] {
            result4 = false;
            temp_in_round += 1;
            break;
        } else {
            temp_in_round += 1;
        }
    }
    multiply(&mut temp, temp_in_round);

    visibles[*r][*c] = temp;
    result1 || result2 || result3 || result4
}
pub fn part1(reader: BufReader<File>) -> (usize, Vec<Vec<usize>>) {
    let mut tree_map = vec![vec![0; 0]; 0];

    for line in BufRead::lines(reader) {
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
            if is_visible_1(&r, &c, &tree_map) {
                visibles += 1;
            }
        }
    }

    (visibles, tree_map)
}

pub fn part2(reader: BufReader<File>) -> usize {
    let (_, tree_map) = part1(reader);

    let mut highest_scenic_score = 0;

    let mut visibles = tree_map.clone();
    visibles.fill(vec![0; tree_map[0].len()]);

    for r in 1..tree_map.len() - 1 {
        for c in 1..tree_map[r].len() - 1 {
            if is_visible_2(&r, &c, &tree_map, &mut visibles) {
                highest_scenic_score = if visibles[r][c] > highest_scenic_score {
                    visibles[r][c]
                } else {
                    highest_scenic_score
                };
            }
        }
    }

    highest_scenic_score
}
