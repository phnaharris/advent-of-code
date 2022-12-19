use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use crate::Circuit;

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut current_circle = 1;
    let mut x = 1;
    let mut result = 0;
    let mut interest_signal_strength_circle = vec![20, 60, 100, 140, 180, 220];

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();
        let circuit = Circuit::from_str(&line).unwrap();

        if interest_signal_strength_circle.contains(&current_circle) {
            result += current_circle * x;
            interest_signal_strength_circle.remove(0);
        }

        match circuit.instruction {
            crate::EInstruction::NOOP => {
                current_circle += 1;
            }
            crate::EInstruction::ADDX => {
                if interest_signal_strength_circle.contains(&(current_circle + 1)) {
                    result += (current_circle + 1) * x;
                    interest_signal_strength_circle.remove(0);
                }

                current_circle += 2;
                x += circuit.offset.unwrap();
            }
        }
    }

    result
}
