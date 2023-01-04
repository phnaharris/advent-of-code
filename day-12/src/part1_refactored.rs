use std::usize::MAX;

use crate::{Dist, Node, Tree};

impl Tree {
    pub fn build_queue(&self) -> (Vec<Node>, Vec<Dist>, Vec<Box<Option<Node>>>) {
        let mut queue = Vec::new();
        let mut dist = Vec::new();
        let mut prev = Vec::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                dist.push(Dist {
                    dist: MAX,
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

        while queue.len() > 0 {
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

        return 0;
    }
}

pub fn find_min(dist: &Vec<Dist>) -> Option<(usize, usize)> {
    if dist.len() == 0 {
        return None;
    }

    let mut min = &dist[0];
    let mut i_min = 0;

    dist.iter().fold(0, |i, val| {
        if val.dist < min.dist {
            min = &*val;
            i_min = i;
        }
        i + 1
    });

    Some((min.dist, i_min))
}

pub fn find_existed_neighbor(queue: &Vec<Node>, node: &Node) -> Vec<Node> {
    let mut iter = queue.iter().filter(|&item| {
        ((item.x as i32 == node.x as i32 && item.y as i32 == node.y as i32 + 1)
            || (item.x as i32 == node.x as i32 && item.y as i32 == node.y as i32 - 1)
            || (item.x as i32 == node.x as i32 + 1 && item.y as i32 == node.y as i32)
            || (item.x as i32 == node.x as i32 - 1 && item.y as i32 == node.y as i32))
            && node.can_go(item.data)
    });

    let mut neighbor = Vec::new();

    loop {
        match iter.next() {
            Some(n) => neighbor.push(n.clone()),
            None => break,
        };
    }

    neighbor
}
