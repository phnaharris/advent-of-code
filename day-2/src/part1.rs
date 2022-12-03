use std::{
    fs::File,
    io::{self, BufReader},
};

fn match_point(opp: char, you: char) -> u8 {
    if opp as u32 - 'A' as u32 == you as u32 - 'X' as u32 {
        3 // round was a draw
    } else if (opp == 'A' && you == 'Z') || (opp == 'B' && you == 'X') || (opp == 'C' && you == 'Y')
    {
        0 // you lose
    } else {
        6 // you win
    }
}

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut total_score: i32 = 0;

    for line in io::BufRead::lines(reader) {
        let mut line = line.unwrap();
        line.remove(1);

        let opp = line.chars().next().unwrap();
        let you = line.chars().nth(1).unwrap();

        let point = match_point(opp, you);

        let your_choice_point = if you == 'X' {
            1 // rock
        } else if you == 'Y' {
            2 // paper
        } else {
            3 // scissors
        };

        total_score += (point + your_choice_point) as i32;
    }

    total_score
}
