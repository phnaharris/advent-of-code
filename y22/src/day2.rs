use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn match_point1(opp: char, you: char) -> u8 {
    if opp as u32 - 'A' as u32 == you as u32 - 'X' as u32 {
        3 // round was a draw
    } else if (opp == 'A' && you == 'Z') || (opp == 'B' && you == 'X') || (opp == 'C' && you == 'Y')
    {
        0 // you lose
    } else {
        6 // you win
    }
}

fn match_point2(wanna: char) -> u8 {
    if wanna == 'Y' {
        3
    } else if wanna == 'X' {
        0
    } else {
        6
    }
}

pub fn part1(reader: BufReader<File>) -> i32 {
    let mut total_score: i32 = 0;

    for line in BufRead::lines(reader) {
        let mut line = line.unwrap();
        line.remove(1);

        let opp = line.chars().next().unwrap();
        let you = line.chars().nth(1).unwrap();

        let point = match_point1(opp, you);

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

fn your_choice(opp: char, wanna: char) -> char {
    if wanna == 'Y' {
        char::from_u32(opp as u32 + ('X' as u32 - 'A' as u32)).unwrap()
    } else if wanna == 'Z' {
        if opp == 'A' {
            'Y'
        } else if opp == 'B' {
            'Z'
        } else {
            'X'
        }
    } else if opp == 'A' {
        'Z'
    } else if opp == 'B' {
        'X'
    } else {
        'Y'
    }
}

pub fn part2(reader: BufReader<File>) -> i32 {
    let mut total_score: i32 = 0;

    for line in BufRead::lines(reader) {
        let mut line = line.unwrap();
        line.remove(1);

        let opp = line.chars().next().unwrap();
        let wanna = line.chars().nth(1).unwrap();

        let your_choice = your_choice(opp, wanna);

        let point = match_point2(wanna);

        let your_choice_point = if your_choice == 'X' {
            1 // rock
        } else if your_choice == 'Y' {
            2 // paper
        } else {
            3 // scissors
        };

        total_score += (point + your_choice_point) as i32;
    }

    total_score
}
