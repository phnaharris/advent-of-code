use std::{
    fs::File,
    io::{self, BufReader},
    str::FromStr,
};

use crate::{Action, LongerRope};

pub fn part1(reader: BufReader<File>) -> usize {
    let mut rope = LongerRope::new(2);

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        let action = Action::from_str(&line).unwrap();

        match action.act {
            'U' => rope.vertical_with_offset(true, action.offset),
            'D' => rope.vertical_with_offset(false, action.offset),
            'L' => rope.horizontal_with_offset(true, action.offset),
            'R' => rope.horizontal_with_offset(false, action.offset),
            _ => println!("are you kidding me huh?"),
        }
    }

    rope.synchronize();

    rope.tail_position_count.len()
}
