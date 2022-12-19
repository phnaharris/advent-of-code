use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
    vec,
};

use crate::Circuit;

const ROW: i32 = 6;
const COL: i32 = 40;

pub fn fill_circle(x: i32, circle: i32, crt: &mut Vec<Vec<char>>) {
    // // sprite position == current_circle - 1; 3
    let circle_row = (circle - 1) / COL;
    let circle_col = (circle - 1) % COL;
    let sprite_pos = vec![x - 1, x, x + 1];

    if sprite_pos.contains(&circle_col) {
        crt[circle_row as usize][circle_col as usize] = '█';
    }
}

pub fn part2(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut current_circle = 1;
    let mut x = 1;
    let mut crt = vec![vec!['.'; COL as usize]; ROW as usize];

    for line in io::BufRead::lines(reader) {
        // start current_circle
        let line = line.unwrap();
        let circuit = Circuit::from_str(&line).unwrap();

        match circuit.instruction {
            crate::EInstruction::NOOP => {
                fill_circle(x, current_circle, &mut crt);
                current_circle += 1;
            }
            crate::EInstruction::ADDX => {
                fill_circle(x, current_circle, &mut crt);
                fill_circle(x, current_circle + 1, &mut crt);
                current_circle += 2;
                x += circuit.offset.unwrap();
            }
        }

        // end current_circle
    }

    for i in 0..ROW {
        for j in 0..COL {
            print!("{}", crt[i as usize][j as usize]);
        }

        println!("");
    }

    crt
}
