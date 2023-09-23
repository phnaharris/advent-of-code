use std::{fs::File, io::BufReader};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn make_reader(input: &str) -> anyhow::Result<(BufReader<File>, BufReader<File>)> {
    let file1 = File::open(input)?;
    let reader1 = BufReader::new(file1);
    let file2 = File::open(input)?;
    let reader2 = BufReader::new(file2);

    anyhow::Ok((reader1, reader2))
}

fn main() -> anyhow::Result<()> {
    let (reader1, reader2) = make_reader("./y21/src/input/day1.txt")?;
    println!("day 1 part 1 result {}", day1::part1(reader1));
    println!("day 1 part 2 result {}", day1::part2(reader2));

    let (reader1, reader2) = make_reader("./y21/src/input/day2.txt")?;
    println!("day 2 part 1 result {}", day2::part1(reader1));
    println!("day 2 part 2 result {}", day2::part2(reader2));

    let (reader1, reader2) = make_reader("./y21/src/input/day3.txt")?;
    println!("day 3 part 1 result {}", day3::part1(reader1));
    println!("day 3 part 2 result {}", day3::part2(reader2));

    let (reader1, reader2) = make_reader("./y21/src/input/day4.txt")?;
    println!("day 4 part 1 result {}", day4::part1(reader1));
    println!("day 4 part 2 result {}", day4::part2(reader2));

    Ok(())
}
