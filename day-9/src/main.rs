mod part1;
mod part2;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    str::FromStr,
    string::ParseError,
    vec,
};

use part1::part1;
use part2::part2;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct LongerRope {
    pub nut: Vec<Point>,
    pub tail_position_count: HashMap<Point, usize>,
}

#[derive(Debug)]
pub struct Action {
    pub act: char,
    pub offset: usize,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split_whitespace().collect();

        Ok(Action {
            act: temp[0].parse().expect("are you kidding me?"),
            offset: temp[1].parse().expect("are you kidding me?"),
        })
    }
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn distance(&self, other: &Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

impl LongerRope {
    pub fn new(length: usize) -> Self {
        let mut default_count = HashMap::new();
        default_count.insert(Point::new(), 1);

        LongerRope {
            nut: vec![Point::new(); length],
            tail_position_count: default_count,
        }
    }

    pub fn distance(&self, i_start: usize, i_end: usize) -> f32 {
        self.nut[i_start].distance(&self.nut[i_end])
    }

    pub fn synchronize(&mut self) {
        for i in 0..self.nut.len() - 1 {
            if self.distance(i, i + 1) < 2.0 {
                continue;
            } else if self.distance(i, i + 1) == 2.0 {
                self.nut[i + 1].x += (self.nut[i].x - self.nut[i + 1].x) / 2;
                self.nut[i + 1].y += (self.nut[i].y - self.nut[i + 1].y) / 2;
            } else {
                self.nut[i + 1].x +=
                    (self.nut[i].x - self.nut[i + 1].x) / (self.nut[i].x - self.nut[i + 1].x).abs();
                self.nut[i + 1].y +=
                    (self.nut[i].y - self.nut[i + 1].y) / (self.nut[i].y - self.nut[i + 1].y).abs();
            }
        }
        *self
            .tail_position_count
            .entry(*self.nut.last().expect("cannot get last element"))
            .or_insert(0) += 1;
    }

    pub fn vertical(&mut self, is_up: bool) {
        self.synchronize();

        if is_up {
            self.nut[0].y += 1;
        } else {
            self.nut[0].y -= 1;
        }
    }

    pub fn vertical_with_offset(&mut self, is_up: bool, offset: usize) {
        for _i in 0..offset {
            self.vertical(is_up);
        }
    }

    pub fn horizontal(&mut self, is_left: bool) {
        self.synchronize();

        if is_left {
            self.nut[0].x -= 1;
        } else {
            self.nut[0].x += 1;
        }
    }

    pub fn horizontal_with_offset(&mut self, is_left: bool, offset: usize) {
        for _i in 0..offset {
            self.horizontal(is_left);
        }
    }
}

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    let positions = part1(reader);
    println!("{:?}", positions);

    // let positions = part2(reader);
    // println!("{:?}", positions);

    Ok(())
}
