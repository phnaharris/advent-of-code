use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::Marker;

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut char_before_marker = 0;
    let mut marker = Marker::new(crate::MarkerType::Packet);

    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();

        for i in 0..line.len() {
            let character = line.chars().nth(i).unwrap();
            marker.add(character);
            char_before_marker += 1;

            if marker.check_is_valid() {
                break;
            }
        }
    }

    char_before_marker
}
