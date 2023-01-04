use crate::Tree;

pub fn part1(tree: &mut Tree) -> usize {
    let len = tree.build2_dijkstra();

    if len == 0 {
        println!("Something went wrong");
    };

    return len;
}
