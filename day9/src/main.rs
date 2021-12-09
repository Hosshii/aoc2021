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

    let solved = solve1(data);
    println!("solved1: {}", solved);
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
