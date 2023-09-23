use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Monkey {
    pub id: usize,
    pub inspect_times: usize,
    pub item: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum EOperation {
    Add,
    Mul,
}

impl FromStr for EOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(EOperation::Mul),
            "+" => Ok(EOperation::Add),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Note {
    pub monkey: Monkey,
    pub op: Operation,
    pub test: Test,
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.inspect_times.partial_cmp(&other.inspect_times) {
            Some(core::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.monkey.partial_cmp(&other.monkey) {
            Some(core::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

#[derive(Clone)]
pub struct Notes {
    pub notes: Vec<Note>,
}

impl Notes {
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }
    pub fn throw(&mut self, from: usize, to: usize, _item: usize) {
        let item = self.notes[from].monkey.item.remove(0);
        self.notes[to].monkey.item.push(item);
    }
    pub fn count_inspect_time(&mut self, count: usize) -> usize {
        self.notes.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let result_vec: Vec<usize> = self.notes[0..count]
            .iter()
            .map(|item| item.monkey.inspect_times)
            .collect();

        let mut result = 1;

        for item in result_vec {
            result *= item;
        }

        result
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Operation {
    pub opcode: EOperation,
    pub operator1: String,
    pub operator2: String,
}

impl Operation {
    pub fn new(opcode: EOperation, operator1: String, operator2: String) -> Self {
        Self {
            opcode,
            operator1,
            operator2,
        }
    }

    pub fn execute(&self, old: usize) -> usize {
        let o1 = if self.operator1 == "old" {
            old
        } else {
            self.operator1.parse().unwrap()
        };

        let o2 = if self.operator2 == "old" {
            old
        } else {
            self.operator2.parse().unwrap()
        };

        let result = match self.opcode {
            EOperation::Add => o1 + o2,
            EOperation::Mul => o1 * o2,
        };

        result / 3
        // result
    }

    pub fn execute2(&self, old: usize, targets: &Vec<usize>) -> usize {
        let mut test_target = 1;
        for item in targets {
            test_target *= item;
        }

        let o1 = if self.operator1 == "old" {
            old
        } else {
            self.operator1.parse().unwrap()
        };

        let o2 = if self.operator2 == "old" {
            old
        } else {
            self.operator2.parse().unwrap()
        };

        match self.opcode {
            EOperation::Add => (o1 + o2) % test_target,
            EOperation::Mul => (o1 * o2) % test_target,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Test {
    pub target: usize,
    pub monkey_true: usize,
    pub monkey_false: usize,
}

impl Test {
    pub fn new(target: usize, monkey_true: usize, monkey_false: usize) -> Self {
        Self {
            target,
            monkey_true,
            monkey_false,
        }
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let lines: Vec<&str> = s.split_inclusive('\n').collect();

        let mut lines = s.split_inclusive('\n');

        let monkey_id = lines
            .next()
            .unwrap()
            .to_string()
            .replace("Monkey", "")
            .replace(':', "")
            .trim()
            .parse::<usize>()
            .unwrap();

        let items: Vec<usize> = lines
            .next()
            .unwrap()
            .to_string()
            .replace("Starting items:", "")
            .trim()
            .split_terminator(',')
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect();

        let mut operation_strs: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let operator2 = operation_strs.pop().unwrap().parse().unwrap();
        let opcode = EOperation::from_str(operation_strs.pop().unwrap());
        let operator1 = operation_strs.pop().unwrap().parse().unwrap();
        let operation = Operation::new(opcode.unwrap(), operator1, operator2);

        let target = lines
            .next()
            .unwrap()
            .to_string()
            .replace("Test: divisible by", "")
            .trim()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let monkey_true = lines
            .next()
            .unwrap()
            .to_string()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let monkey_false = lines
            .next()
            .unwrap()
            .to_string()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let result = Note::new(
            Monkey::new(monkey_id, 0, items),
            operation,
            Test::new(target, monkey_true, monkey_false),
        );

        Ok(result)
    }
}

impl Monkey {
    pub fn new(id: usize, inspect_times: usize, item: Vec<usize>) -> Self {
        Self {
            id,
            inspect_times,
            item,
        }
    }

    pub fn inspect(&mut self, times: usize) {
        self.inspect_times += times;
    }
}

impl Note {
    pub fn new(monkey: Monkey, op: Operation, test: Test) -> Self {
        Self { monkey, op, test }
    }

    pub fn test(&self, item: usize) -> bool {
        item % self.test.target == 0
    }
}

fn data_preprocessing(file_str: String) -> Notes {
    let mut notes = Vec::new();

    let monkey_strs: Vec<&str> = file_str.split("\n\n").collect();
    for item in monkey_strs {
        let new_note = Note::from_str(item).expect("expect return new note");
        notes.push(new_note);
    }

    Notes::new(notes)
}

pub fn process_monkey(notes: &mut Notes, monkey_id: usize) {
    notes.notes[monkey_id].monkey.item = notes.notes[monkey_id]
        .monkey
        .item
        .iter()
        .map(|i| notes.notes[monkey_id].op.execute(*i))
        .collect();

    let count = notes.notes[monkey_id].monkey.item.len();
    notes.notes[monkey_id].monkey.inspect(count);

    loop {
        if notes.notes[monkey_id].monkey.item.is_empty() {
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

pub fn process_monkey_2(notes: &mut Notes, monkey_id: usize) {
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
        if notes.notes[monkey_id].monkey.item.is_empty() {
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

pub fn play_one_round_1(notes: &mut Notes) {
    for monkey_id in 0..notes.notes.len() {
        process_monkey(notes, monkey_id)
    }
}

pub fn play_one_round_2(notes: &mut Notes) {
    for monkey_id in 0..notes.notes.len() {
        process_monkey_2(notes, monkey_id)
    }
}

pub fn play_1(notes: &mut Notes, round: &mut usize) {
    while round > &mut 0 {
        play_one_round_1(notes);
        *round -= 1;
    }
}

pub fn play_2(notes: &mut Notes, round: &mut usize) {
    while round > &mut 0 {
        play_one_round_2(notes);
        *round -= 1;
    }
}

pub fn part1(reader: &mut BufReader<File>) -> usize {
    let mut file_str = String::new();
    reader.read_to_string(&mut file_str).unwrap();
    let mut notes = data_preprocessing(file_str);
    play_1(&mut notes, &mut 20);
    notes.count_inspect_time(2)
}

pub fn part2(reader: &mut BufReader<File>) -> usize {
    let mut file_str = String::new();
    reader.read_to_string(&mut file_str).unwrap();
    let mut notes = data_preprocessing(file_str);
    play_2(&mut notes, &mut 10000);
    notes.count_inspect_time(2)
}
