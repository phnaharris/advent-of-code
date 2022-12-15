use crate::{Action, Stack};

pub fn part1(stacks: &mut Vec<Stack>, actions: Vec<Action>) -> String {
    for action in actions {
        let count = action.count;
        let from = action.from;
        let to = action.to;

        for _i in 0..count {
            let temp = stacks[from].crates.pop().unwrap();
            stacks[to].crates.push(temp);
        }
    }

    let mut top_crates = String::from("");

    (1..stacks.len()).for_each(|i| {
        top_crates.push(*stacks[i].crates.last().unwrap());
    });

    top_crates
}
