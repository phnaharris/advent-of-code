use std::{fs::File, io::BufReader};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn make_reader(input: &str) -> anyhow::Result<(BufReader<File>, BufReader<File>)> {
    let file1 = File::open(input)?;
    let reader1 = BufReader::new(file1);
    let file2 = File::open(input)?;
    let reader2 = BufReader::new(file2);

    anyhow::Ok((reader1, reader2))
}

fn main() -> anyhow::Result<()> {
    let (reader1, reader2) = make_reader("./y22/src/input/day1.txt")?;
    println!("day 1 part 1 result {}", day1::part1(reader1));
    println!("day 1 part 2 result {}", day1::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day2.txt")?;
    println!("day 2 part 1 result {}", day2::part1(reader1));
    println!("day 2 part 2 result {}", day2::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day3.txt")?;
    println!("day 3 part 1 result {}", day3::part1(reader1));
    println!("day 3 part 2 result {}", day3::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day4.txt")?;
    println!("day 4 part 1 result {}", day4::part1(reader1));
    println!("day 4 part 2 result {}", day4::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day5.txt")?;
    println!("day 5 part 1 result {}", day5::part1(reader1));
    println!("day 5 part 2 result {}", day5::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day6.txt")?;
    println!("day 6 part 1 result {}", day6::part1(reader1));
    println!("day 6 part 2 result {}", day6::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day7.txt")?;
    println!("day 7 part 1 result {}", day7::part1(reader1).0);
    println!("day 7 part 2 result {}", day7::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day8.txt")?;
    println!("day 8 part 1 result {}", day8::part1(reader1).0);
    println!("day 8 part 2 result {}", day8::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day9.txt")?;
    println!("day 9 part 1 result {}", day9::part1(reader1));
    println!("day 9 part 2 result {}", day9::part2(reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day10.txt")?;
    println!("day 10 part 1 result {}", day10::part1(reader1));
    println!("day 10 part 2 result {:?}", day10::part2(reader2));

    let (mut reader1, mut reader2) = make_reader("./y22/src/input/day11.txt")?;
    println!("day 11 part 1 result {}", day11::part1(&mut reader1));
    println!("day 11 part 2 result {}", day11::part2(&mut reader2));

    let (reader1, reader2) = make_reader("./y22/src/input/day12.txt")?;
    println!("day 12 part 1 result {}", day12::part1(reader1));
    println!("day 12 part 2 result {}", day12::part2(reader2));

    Ok(())
}
