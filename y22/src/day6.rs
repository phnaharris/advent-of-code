use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum MarkerType {
    Packet,
    Message,
}

#[derive(Debug)]
struct Marker {
    value: Vec<char>,
    marker_type: MarkerType,
}

impl Marker {
    fn new(t: MarkerType) -> Self {
        Self {
            value: vec![' '; 0],
            marker_type: t,
        }
    }

    fn add(&mut self, c: char) -> &Self {
        let _ = &self.value.push(c);

        let max_char = match &self.marker_type {
            MarkerType::Packet => 4,
            MarkerType::Message => 14,
        };

        if self.value.len() > max_char {
            let _ = &self.value.remove(0);
        }

        self
    }

    fn check_is_valid(&self) -> bool {
        let max_char = match &self.marker_type {
            MarkerType::Packet => 4,
            MarkerType::Message => 14,
        };

        if self.value.len() != max_char {
            return false;
        };

        let mut tracing: HashMap<char, i32> = HashMap::new();

        for c in &self.value {
            let count = tracing.get(c);

            match count {
                Some(_) => return false,
                None => tracing.insert(*c, 1),
            };
        }

        true
    }
}

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut char_before_marker = 0;
    let mut marker = Marker::new(MarkerType::Packet);

    for line in BufRead::lines(reader) {
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

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut char_before_marker = 0;
    let mut marker = Marker::new(MarkerType::Message);

    for line in BufRead::lines(reader) {
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
