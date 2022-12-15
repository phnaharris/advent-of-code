use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

use crate::FolderStack;

pub fn part1(reader: BufReader<File>) -> (usize, HashMap<String, usize>) {
    let mut total_size = 0;

    let mut child_folders: HashMap<String, usize> = HashMap::new(); // Map<Path, Size>
    let mut current_folder = FolderStack::new();

    for line in io::BufRead::lines(reader) {
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
