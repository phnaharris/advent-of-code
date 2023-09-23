use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut matrix = vec![vec!['x'; 0]; 0];

    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        let mut s = vec!['c'; 0];

        for c in line.chars() {
            s.push(c);
        }

        matrix.push(s);
    }

    matrix
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn find_coord(matrix: &Vec<Vec<char>>, c: char) -> Option<Point> {
    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if c == matrix[x][y] {
                return Some(Point { x, y });
            }
        }
    }

    None
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub len: usize,
    pub data: char,
    pub children: Vec<Node>,
    pub parent: Vec<usize>, // store just id to trace back: id = row * row_length + col
}

#[derive(Debug)]
pub struct Dist {
    pub dist: usize,
    pub id: usize,
}

pub fn is_went(x: usize, y: usize, row_length: usize, parent: &[usize]) -> bool {
    parent.iter().any(|&item| item == x * row_length + y)
}

impl Node {
    pub fn new(x: usize, y: usize, len: usize, c: char, parent: Vec<usize>) -> Self {
        Self {
            x,
            y,
            len,
            data: c,
            children: Vec::new(),
            parent,
        }
    }

    pub fn get_node_id(&self, row_length: usize) -> usize {
        self.x * row_length + self.y
    }

    pub fn distance(&self, c: char) -> i32 {
        let temp_src = if self.data == 'S' {
            'a'
        } else if self.data == 'E' {
            'z'
        } else {
            self.data
        };
        let temp_dest = if c == 'S' {
            'a'
        } else if c == 'E' {
            'z'
        } else {
            c
        };

        temp_dest as i32 - temp_src as i32
    }
    pub fn can_go(&self, c: char) -> bool {
        self.distance(c) <= 1
    }

    pub fn add_path(&mut self, matrix: &Vec<Vec<char>>) -> anyhow::Result<()> {
        let x = self.x;
        let y = self.y;
        let row_length = matrix[0].len();

        let mut parent = self.parent.clone();
        parent.push(x * matrix[0].len() + y);

        if x < matrix.len() - 1
            && self.can_go(matrix[x + 1][y])
            && !is_went(x + 1, y, row_length, &self.parent)
        {
            self.children.push(Node::new(
                x + 1,
                y,
                self.len + 1,
                matrix[x + 1][y],
                parent.clone(),
            ))
        }
        if x > 0 && self.can_go(matrix[x - 1][y]) && !is_went(x - 1, y, row_length, &self.parent) {
            self.children.push(Node::new(
                x - 1,
                y,
                self.len + 1,
                matrix[x - 1][y],
                parent.clone(),
            ))
        }
        if y < matrix[0].len() - 1
            && self.can_go(matrix[x][y + 1])
            && !is_went(x, y + 1, row_length, &self.parent)
        {
            self.children.push(Node::new(
                x,
                y + 1,
                self.len + 1,
                matrix[x][y + 1],
                parent.clone(),
            ))
        }
        if y > 0 && self.can_go(matrix[x][y - 1]) && !is_went(x, y - 1, row_length, &self.parent) {
            self.children
                .push(Node::new(x, y - 1, self.len + 1, matrix[x][y - 1], parent))
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub head: Node,
    pub matrix: Vec<Vec<char>>,
}

impl Tree {
    pub fn new(matrix: Vec<Vec<char>>) -> Self {
        let start = find_coord(&matrix, 'S').expect("input has no start point");
        let x = start.x;
        let y = start.y;

        Self {
            head: Node {
                x,
                y,
                len: 0,
                data: matrix[x][y],
                children: Vec::new(),
                parent: Vec::new(),
            },
            matrix,
        }
    }

    pub fn build_normal(&mut self) -> usize {
        self.head.add_path(&self.matrix).expect("ahihi");

        let mut queue = Vec::new();

        queue.extend(self.head.children.iter_mut());

        loop {
            let current = queue.remove(0);

            if current.data.eq(&'E') {
                current.parent.sort();
                return current.parent.len();

                // for i in 0..5 {
                //     for j in 0..8 {
                //         if current.parent[0] == i * 8 + j {
                //             print!("+");
                //             current.parent.remove(0);
                //         } else {
                //             print!("-");
                //         }
                //     }
                // }
            }

            current.add_path(&self.matrix).expect("ahihi");
            if let Some(index) = current
                .children
                .iter()
                .position(|item| item.x == current.x && item.y == current.y)
            {
                current.children.swap_remove(index);
            }

            queue.extend(current.children.iter_mut());

            if queue.is_empty() {
                return 0;
            }
        }
    }
    pub fn build(&mut self) -> usize {
        self.head.add_path(&self.matrix).expect("ahihi");

        let mut queue = Vec::new();

        queue.par_extend(self.head.children.par_iter_mut());

        loop {
            let current = queue.remove(0);

            if current.data.eq(&'E') {
                current.parent.sort();
                return current.parent.len();

                // for i in 0..5 {
                //     for j in 0..8 {
                //         if current.parent[0] == i * 8 + j {
                //             print!("+");
                //             current.parent.remove(0);
                //         } else {
                //             print!("-");
                //         }
                //     }
                // }
            }

            rayon::join(
                || {
                    current.add_path(&self.matrix).expect("ahihi");
                    if let Some(index) = current
                        .children
                        .par_iter()
                        .position_first(|item| item.x == current.x && item.y == current.y)
                    {
                        current.children.swap_remove(index);
                    }

                    queue.par_extend(current.children.par_iter_mut());
                },
                || {},
            );

            if queue.is_empty() {
                return 0;
            }
        }
    }
    pub fn build_queue(&self) -> (Vec<Node>, Vec<Dist>, Vec<Box<Option<Node>>>) {
        let mut queue = Vec::new();
        let mut dist = Vec::new();
        let mut prev = Vec::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                dist.push(Dist {
                    dist: usize::MAX,
                    id: i * self.matrix[0].len() + j,
                });
                prev.push(Box::new(None));

                let new_node = Node::new(i, j, 0, self.matrix[i][j], Vec::new());
                queue.push(new_node);
            }
        }

        (queue, dist, prev)
    }

    pub fn build2_dijkstra(&mut self) -> usize {
        let (mut queue, mut dist, mut prev) = self.build_queue();
        let head_id = self.head.get_node_id(self.matrix[0].len());
        dist[head_id] = Dist {
            dist: 0,
            id: head_id,
        };
        let mut res_dist = Vec::new();

        while !queue.is_empty() {
            let (min, i_min) = find_min(&dist).unwrap();

            let u = queue.remove(i_min);
            let d = dist.remove(i_min);
            res_dist.push(min);

            if u.data == 'E' {
                return min;
            }

            let neighbor = find_existed_neighbor(&queue, &u);

            for nei in neighbor {
                let alt = d.dist + 1;
                let nei_id = nei.x * self.matrix[0].len() + nei.y;
                let dist_nei_id = dist.iter().position(|item| item.id == nei_id).unwrap();
                if alt < dist[dist_nei_id].dist {
                    dist[dist_nei_id].dist = alt;
                    prev[nei_id] = Box::new(Some(u.clone()));
                }
            }
        }

        0
    }

    pub fn part2_dijkstra(&mut self) -> usize {
        let (mut queue, mut dist, mut prev) = self.build_queue();

        queue.iter().fold(0, |i, val| {
            if val.data == 'a' {
                dist[i] = Dist { dist: 0, id: i }
            }
            i + 1
        });

        let mut res_dist = Vec::new();

        while !queue.is_empty() {
            let (min, i_min) = find_min(&dist).unwrap();

            let u = queue.remove(i_min);
            let d = dist.remove(i_min);
            res_dist.push(min);

            if u.data == 'E' {
                return min;
            }

            let neighbor = find_existed_neighbor(&queue, &u);

            for nei in neighbor {
                let alt = d.dist + 1;
                let nei_id = nei.x * self.matrix[0].len() + nei.y;
                let dist_nei_id = dist.iter().position(|item| item.id == nei_id).unwrap();
                if alt < dist[dist_nei_id].dist {
                    dist[dist_nei_id].dist = alt;
                    prev[nei_id] = Box::new(Some(u.clone()));
                }
            }
        }

        0
    }
}

pub fn find_min(dist: &Vec<Dist>) -> Option<(usize, usize)> {
    if dist.is_empty() {
        return None;
    }

    let mut min = &dist[0];
    let mut i_min = 0;

    dist.iter().fold(0, |i, val| {
        if val.dist < min.dist {
            min = val;
            i_min = i;
        }
        i + 1
    });

    Some((min.dist, i_min))
}

pub fn find_existed_neighbor(queue: &[Node], node: &Node) -> Vec<Node> {
    let iter = queue.iter().filter(|&item| {
        (item.x as i32 == node.x as i32 && item.y as i32 == node.y as i32 + 1
            || (item.x as i32 == node.x as i32 && item.y as i32 == node.y as i32 - 1)
            || (item.x as i32 == node.x as i32 + 1 && item.y as i32 == node.y as i32)
            || (item.x as i32 == node.x as i32 - 1 && item.y as i32 == node.y as i32))
            && node.can_go(item.data)
    });

    let mut neighbor = Vec::new();

    for n in iter {
        neighbor.push(n.clone())
    }

    neighbor
}
pub fn part1(reader: BufReader<File>) -> usize {
    let matrix = read_input(reader);
    let mut tree = Tree::new(matrix);

    let len = tree.build2_dijkstra();

    if len == 0 {
        println!("Something went wrong");
    };

    len
}

pub fn part2(reader: BufReader<File>) -> usize {
    let matrix = read_input(reader);
    let mut tree = Tree::new(matrix);
    let len = tree.part2_dijkstra();

    if len == 0 {
        println!("Something went wrong");
    };

    len
}
