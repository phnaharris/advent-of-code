use crate::Notes;

pub fn process_monkey(notes: &mut Notes, monkey_id: usize) {
    let targets = notes.notes.iter().map(|item| item.test.target).collect();

    notes.notes[monkey_id].monkey.item = notes.notes[monkey_id]
        .monkey
        .item
        .iter()
        .map(|i| notes.notes[monkey_id].op.execute2(*i, &targets))
        .collect();

    let count = notes.notes[monkey_id].monkey.item.len();
    notes.notes[monkey_id].monkey.inspect(count);

    loop {
        if notes.notes[monkey_id].monkey.item.len() == 0 {
            break;
        }

        let item = notes.notes[monkey_id].monkey.item[0];

        if notes.notes[monkey_id].test(item) {
            notes.throw(monkey_id, notes.notes[monkey_id].test.monkey_true, item);
        } else {
            notes.throw(monkey_id, notes.notes[monkey_id].test.monkey_false, item);
        }
    }
}

pub fn play_one_round(notes: &mut Notes) {
    for monkey_id in 0..notes.notes.len() {
        process_monkey(notes, monkey_id)
    }
}

pub fn play(notes: &mut Notes, round: &mut usize) {
    while round > &mut 0 {
        play_one_round(notes);
        *round -= 1;
    }
}

pub fn part2(notes: &mut Notes) -> usize {
    play(notes, &mut 10000);
    notes.count_inspect_time(2)
}
