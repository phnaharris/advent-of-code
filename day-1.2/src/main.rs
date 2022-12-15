use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;
    let mut count = 0;
    let mut last = 9999999;
    let continous_element = 3;

    let mut current_sum = vec![0; 0];

    let reader = BufReader::new(file);
    for line in io::BufRead::lines(reader) {
        let current = line?.parse::<i32>().unwrap();

        if current_sum.len() < continous_element {
            current_sum.push(current);
            continue;
        } else {
            last = current_sum.iter().sum::<i32>();

            current_sum.remove(0);
            current_sum.push(current);
        }

        if current_sum.iter().sum::<i32>() > last {
            count += 1;
        }
    }

    println!("{:?}", count);

    Ok(())
}
