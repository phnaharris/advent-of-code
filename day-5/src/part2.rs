use crate::{Action, Stack};

pub fn part2(stacks: &mut Vec<Stack>, actions: Vec<Action>) -> String {
    for action in actions {
        let count = action.count;
        let from = action.from;
        let to = action.to;

        let length = stacks[from].crates.len();
        let temp: &mut [char] = &mut *stacks[from]
            .crates
            .drain(length - count..)
            .collect::<Vec<char>>();

        stacks[to].crates.append(&mut Vec::from(temp));
    }

    let mut top_crates = String::from("");

    (1..stacks.len()).for_each(|i| {
        top_crates.push(*stacks[i].crates.last().unwrap());
    });

    top_crates
}
