use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input");
    let mut parsed = parse(input);
    parsed[0][0] = 0;
    let solved = solve(&parsed);
    println!("{}", solved);
}

fn parse(ipt: &str) -> Vec<Vec<u8>> {
    ipt.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

const DIRECTION: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve(map: &[Vec<u8>]) -> u64 {
    let mut v = vec![vec![u64::MAX; map[0].len()]; map.len()];
    v[0][0] = 0;

    let mut cur = (0, 0);
    let mut determined = HashSet::new();

    let mut not_determined = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            not_determined.insert((x, y));
        }
    }

    while !not_determined.is_empty() {
        cur = find_min(&v, &not_determined);
        // println!("{:?}", cur);
        not_determined.remove(&cur);
        determined.insert(cur);

        let (cur_x, cur_y) = cur;
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

            let current_cost = v[cur_y][cur_x];
            let neighbor_cost_cur = v[neighbor_y][neighbor_x];
            let neighbor_cost_next = current_cost + map[neighbor_y][neighbor_x] as u64;
            if neighbor_cost_next < neighbor_cost_cur {
                v[neighbor_y][neighbor_x] = neighbor_cost_next;
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
