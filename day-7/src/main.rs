mod part1;
mod part2;

use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{self, BufReader},
};

const MAX_SPACE: usize = 70000000;
const LEAST_UNUSED_SPACE: usize = 30000000;

use part1::part1;
use part2::part2;

#[derive(Clone, Debug)]
pub struct Folder {
    pub path: String,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct FolderStack(VecDeque<String>);

impl FolderStack {
    pub fn new() -> Self {
        FolderStack(VecDeque::new())
    }

    pub fn push_back(&mut self, folder: String) {
        self.0.push_back(folder);
    }

    pub fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }
}

impl Into<String> for FolderStack {
    fn into(mut self) -> String {
        String::from(self.0.make_contiguous().join("/"))
    }
}

fn main() -> io::Result<()> {
    let input = String::from("./input/input.txt");
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    let (total_size, folder_tree) = part1(reader);
    println!("{:?}", total_size);

    let total_smallest_dir = part2(folder_tree);
    println!("{:?}", total_smallest_dir);

    Ok(())
}
