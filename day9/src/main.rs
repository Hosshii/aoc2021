use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let ipt = File::open("input")?;
    let buf = BufReader::new(ipt);
    let data = buf
        .lines()
        .collect::<std::result::Result<Vec<String>, std::io::Error>>();
    let data = data?
        .iter()
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or("err")?;

    let data: Vec<_> = data
        .into_iter()
        .map(|v| v.into_iter().map(|v| (v, false)).collect())
        .collect();
    let solved = solve2(data);
    println!("solved2: {}", solved);
    Ok(())
}

fn solve1(data: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let t = if y == 0 { None } else { Some(data[y - 1][x]) };
            let b = if y == data.len() - 1 {
                None
            } else {
                Some(data[y + 1][x])
            };

            let r = if x == row.len() - 1 {
                None
            } else {
                Some(row[x + 1])
            };

            let l = if x == 0 { None } else { Some(row[x - 1]) };

            if let Some(x) = [t, b, r, l].iter().fold(Some(*col), |acc, x| {
                acc.and_then(|acc| match x {
                    Some(x) => {
                        if acc < *x {
                            Some(acc)
                        } else {
                            None
                        }
                    }
                    None => Some(acc),
                })
            }) {
                result += x as u32 + 1;
            }
        }
    }
    result
}

// data (value, is_seen)
fn solve2(mut data: Vec<Vec<(u32, bool)>>) -> u32 {
    let mut result = Vec::new();
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if is_valid(&data, (x, y)) {
                let size = count_size(&mut data, (x, y));

                result.push(size);
            }
        }
    }

    result.sort_unstable();
    let len = result.len();

    result[len - 1] * result[len - 2] * result[len - 3]
}

fn count_size(data: &mut Vec<Vec<(u32, bool)>>, (x, y): (usize, usize)) -> u32 {
    let mut result = 1;
    data[y][x].1 = true;

    // up
    if y != 0 {
        let up = (x, y - 1);
        if is_valid(data, up) {
            result += count_size(data, up);
        }
    }

    // bottom
    if y != data.len() - 1 {
        let bottom = (x, y + 1);
        if is_valid(data, bottom) {
            result += count_size(data, bottom);
        }
    }

    // right
    if x != data[0].len() - 1 {
        let right = (x + 1, y);
        if is_valid(data, right) {
            result += count_size(data, right);
        }
    }

    // left
    if x != 0 {
        let left = (x - 1, y);
        if is_valid(data, left) {
            result += count_size(data, left);
        }
    }
    result
}

fn is_valid(data: &[Vec<(u32, bool)>], (x, y): (usize, usize)) -> bool {
    !data[y][x].1 && data[y][x].0 < 9
}
