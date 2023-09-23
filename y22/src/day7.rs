use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_SPACE: usize = 70000000;
const LEAST_UNUSED_SPACE: usize = 30000000;

#[derive(Clone, Debug)]
pub struct Folder {
    pub path: String,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct FolderStack(VecDeque<String>);

impl Default for FolderStack {
    fn default() -> Self {
        Self::new()
    }
}

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

impl From<FolderStack> for String {
    fn from(mut val: FolderStack) -> Self {
        val.0.make_contiguous().join("/")
    }
}

pub fn part1(reader: BufReader<File>) -> (usize, HashMap<String, usize>) {
    let mut total_size = 0;

    let mut child_folders: HashMap<String, usize> = HashMap::new(); // Map<Path, Size>
    let mut current_folder = FolderStack::new();

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let lines: Vec<&str> = line.split_whitespace().collect();

        let temp_current_folder = &mut current_folder;

        match lines[0] {
            "$" => {
                // command
                if let "cd" = lines[1] {
                    if let ".." = lines[2] {
                        temp_current_folder.pop_back();
                    } else {
                        temp_current_folder.push_back(lines[2].to_string());
                    }
                } else if "ls" == lines[1] {
                }
            }
            "dir" => {
                // folder
                let new_folder = lines[1];

                let mut temp_path = temp_current_folder.clone();
                temp_path.push_back(new_folder.to_string());

                child_folders.entry(temp_path.into()).or_insert(0);
            }
            _ => {
                // files
                let file_size: usize = lines[0].parse().unwrap();

                let tmp = temp_current_folder.clone();

                let mut temp_cur_path = String::from("");
                for item in tmp.0 {
                    let temp_cur_path = &mut temp_cur_path;
                    temp_cur_path.push_str(&item);
                    *child_folders.entry(temp_cur_path.to_string()).or_insert(0) += file_size;
                }
            }
        }
    }

    for (_k, v) in child_folders.iter().filter(|&(_k, v)| v <= &100000) {
        total_size += v;
    }

    (total_size, child_folders)
}

pub fn part2(reader: BufReader<File>) -> usize {
    let (_, folder_tree) = part1(reader);

    let current_root_size = folder_tree.get(&"/".to_string()).unwrap();
    let current_unused = MAX_SPACE - current_root_size;
    let need_space = LEAST_UNUSED_SPACE - current_unused;

    let mut total_smallest_dir = usize::MAX;
    for item in folder_tree {
        if item.1 > need_space && item.1 < total_smallest_dir {
            total_smallest_dir = item.1;
        }
    }

    total_smallest_dir
}
