use std::{collections::HashMap, usize::MAX};

use crate::{LEAST_UNUSED_SPACE, MAX_SPACE};

pub fn part2(folder_tree: HashMap<String, usize>) -> usize {
    let current_root_size = folder_tree.get(&"/".to_string()).unwrap();
    let current_unused = MAX_SPACE - current_root_size;
    let need_space = LEAST_UNUSED_SPACE - current_unused;

    let mut total_smallest_dir = MAX;
    for item in folder_tree {
        if item.1 > need_space && item.1 < total_smallest_dir {
            total_smallest_dir = item.1;
        }
    }

    total_smallest_dir
}
