use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input/input");
    let parsed = parse(input);
    let solved = solve(parsed);
    println!("{}", solved);
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c != '0').collect::<Vec<bool>>())
        .collect()
}

fn solve(data: Vec<Vec<bool>>) -> u32 {
    let mut gamma = data.clone();
    let mut idx = 0;
    while gamma.len() > 1 {
        let b = match (gamma.len() as i32).cmp(&(count_bit(&gamma, idx) * 2)) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => true,
        };
        gamma = gamma.iter().cloned().filter(|v| v[idx] == b).collect();
        idx += 1;
    }

    let mut epsilon = data;
    let mut idx = 0;
    while epsilon.len() > 1 {
        let b = match (epsilon.len() as i32).cmp(&(count_bit(&epsilon, idx) * 2)) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => false,
        };
        epsilon = epsilon.iter().cloned().filter(|v| v[idx] == b).collect();
        idx += 1;
    }

    let gamma_n = to_dec(&gamma[0]);
    let epsilon_n = to_dec(&epsilon[0]);

    gamma_n * epsilon_n
}

fn count_bit(data: &[Vec<bool>], n: usize) -> i32 {
    let mut count = 0;
    for row in data {
        if row[n] {
            count += 1;
        }
    }
    count
}

fn to_dec(ipt: &[bool]) -> u32 {
    let mut res = 0;
    let ipt = ipt.iter().skip_while(|b| !*b).collect::<Vec<_>>();

    for (idx, &&i) in ipt.iter().rev().enumerate() {
        let mut i = i as u32;
        i <<= idx;
        res |= i;
    }
    res
}
