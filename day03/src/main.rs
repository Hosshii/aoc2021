fn main() {
    let input = include_str!("../input/input");
    let parsed = parse(input);
    let solved = solve(&parsed);
    println!("{}", solved);
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c != '0').collect::<Vec<bool>>())
        .collect()
}

fn solve(data: &[Vec<bool>]) -> u32 {
    let mut gamma = Vec::new();
    let cols = data[0].len();
    for i in 0..cols {
        let mut zeros = 0;
        for row in data {
            if !row[i] {
                zeros += 1;
            }
        }
        if data.len() as isize - 2 * zeros > 0 {
            gamma.push(true);
        } else {
            gamma.push(false);
        }
    }
    let epsilon = gamma.iter().map(|c| !*c).collect::<Vec<bool>>();
    let gamma_n = to_dec(&gamma);
    let epsilon_n = to_dec(&epsilon);

    gamma_n * epsilon_n
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
