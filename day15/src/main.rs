use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let input = include_str!("../input/input");
    let mut parsed = parse(input);
    parsed[0][0] = 0;
    let solved = solve(&parsed);
    println!("{}", solved);
}

fn parse(ipt: &str) -> Vec<Vec<u8>> {
    let orig = ipt
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    for y in 0..5 {
        for rows in orig.iter() {
            let mut row = Vec::new();
            for x in 0..5 {
                for col in rows {
                    row.push(((col + x + y - 1) % 9) + 1);
                }
            }
            result.push(row);
        }
    }
    result
}

const DIRECTION: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: (usize, usize),
    cost: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn solve(map: &[Vec<u8>]) -> u64 {
    let mut v = vec![vec![u64::MAX; map[0].len()]; map.len()];
    v[0][0] = 0;

    let mut not_determined = BinaryHeap::new();
    not_determined.push(Reverse(State {
        pos: (0, 0),
        cost: 0,
    }));

    while let Some(Reverse(State { pos, cost })) = not_determined.pop() {
        let (cur_x, cur_y) = pos;
        for (dx, dy) in DIRECTION {
            let (neighbor_x, neighbor_y) = (cur_x as isize + dx, cur_y as isize + dy);
            if neighbor_x < 0
                || neighbor_y < 0
                || neighbor_x >= map[0].len() as isize
                || neighbor_y >= map.len() as isize
            {
                continue;
            }
            let (neighbor_x, neighbor_y) = (neighbor_x as usize, neighbor_y as usize);

            let current_cost = cost;
            let neighbor_cost_cur = v[neighbor_y][neighbor_x];
            let neighbor_cost_next = current_cost + map[neighbor_y][neighbor_x] as u64;
            if neighbor_cost_next < neighbor_cost_cur {
                v[neighbor_y][neighbor_x] = neighbor_cost_next;
                not_determined.push(Reverse(State {
                    pos: (neighbor_x, neighbor_y),
                    cost: neighbor_cost_next,
                }));
            }
        }
    }

    v[v.len() - 1][v[0].len() - 1]
}

fn find_min(map: &[Vec<u64>], set: &HashSet<(usize, usize)>) -> (usize, usize) {
    let mut min = u64::MAX;
    let mut result = (0, 0);
    for (x, y) in set {
        if map[*y][*x] < min {
            result = (*x, *y);
            min = map[*y][*x];
        }
    }
    result
}
