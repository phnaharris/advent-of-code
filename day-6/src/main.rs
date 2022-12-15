mod part1;
mod part2;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

use part1::part1;
use part2::part2;

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

        if &self.value.len() > &max_char {
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
            let count = tracing.get(&c);

            match count {
                Some(counter) => return false,
                None => tracing.insert(*c, 1),
            };
        }

        return true;
    }
}

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    // let char_before_marker = part1(reader);
    // println!("{:?}", char_before_marker);

    let char_before_marker = part2(reader);
    println!("{:?}", char_before_marker);

    Ok(())
}
