use crate::{
    part1_refactored::{find_existed_neighbor, find_min},
    Dist, Tree,
};

impl Tree {
    pub fn part2_dijkstra(&mut self) -> usize {
        let (mut queue, mut dist, mut prev) = self.build_queue();

        queue.iter().fold(0, |i, val| {
            if val.data == 'a' {
                dist[i] = Dist { dist: 0, id: i }
            }
            i + 1
        });

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

pub fn part2(tree: &mut Tree) -> usize {
    let len = tree.part2_dijkstra();

    if len == 0 {
        println!("Something went wrong");
    };

    return len;
}
