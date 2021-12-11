fn main() {
    let ipt = include_str!("../input/input");
    let parsed = parse(ipt);
    let mut parsed = parsed
        .into_iter()
        .map(|x| x.into_iter().map(|c| (c, false)).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut sum = 0;
    for i in 0..100 {
        sum += step(&mut parsed);
    }

    println!("{}", sum);
}

fn parse(ipt: &str) -> Vec<Vec<i32>> {
    ipt.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn step(data: &mut [Vec<(i32, bool)>]) -> i32 {
    let direction = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for row in data.iter_mut() {
        for (c, _) in row.iter_mut() {
            *c += 1;
        }
    }

    while check_flash(data) {
        for y in 0..data.len() {
            for x in 0..data[y].len() {
                if !data[y][x].1 && data[y][x].0 > 9 {
                    data[y][x].1 = true;
                    for (dx, dy) in direction.iter() {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        incr(data, nx, ny, 1);
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for row in data.iter_mut() {
        for (c, b) in row.iter_mut() {
            if *c > 9 && *b {
                sum += 1;
                *c = 0;
                *b = false;
            }
        }
    }
    sum
}

fn incr(data: &mut [Vec<(i32, bool)>], x: isize, y: isize, n: i32) {
    if y < 0 || x < 0 || y >= data.len() as isize || x >= data[y as usize].len() as isize {
        return;
    }
    data[y as usize][x as usize].0 += n;
}

/// まだ点灯していない箇所があるかどうか
fn check_flash(data: &mut [Vec<(i32, bool)>]) -> bool {
    data.iter()
        .any(|row| row.iter().any(|(c, b)| (*c > 9) && !*b))
}
