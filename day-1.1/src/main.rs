use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;
    let mut count = 0;
    let mut last = 9999999;

    let reader = BufReader::new(file);
    for line in io::BufRead::lines(reader) {
        let current = line?.parse::<i32>().unwrap();

        if current > last {
            count += 1;
        }

        last = current;
    }

    println!("{:?}", count);

    Ok(())
}
