use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let mut horizontal = 0;
    let mut depth = 0;

    let reader = BufReader::new(file);
    for line in io::BufRead::lines(reader) {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        // let item: Vec<&str> = split.collect();

        let action = String::from(split[0]);
        let count = split[1].parse::<i32>().unwrap();

        if action == *"forward" {
            horizontal += count;
        } else if action == *"down" {
            depth += count;
        } else {
            depth -= count;
        }
    }

    println!("horizontal: {:?}", horizontal);
    println!("depth: {:?}", depth);
    println!("Result: {:?}", horizontal * depth);

    Ok(())
}
