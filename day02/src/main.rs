use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    println!("Hello, world!");
    let ipt = include_str!("../input/input");
    let parsed = parse(ipt)?;
    let solved = solve(&parsed);
    println!("{}", solved);
    Ok(())
}

fn solve(ipt: &[(Direction, i32)]) -> i32 {
    let (h, d, _) = ipt
        .iter()
        .fold((0, 0, 0), |(horizontal, depth, aim), (d, x)| match d {
            Direction::Down => (horizontal, depth, aim + x),
            Direction::Up => (horizontal, depth, aim - x),
            Direction::Forward => (horizontal + x, depth + x * aim, aim),
        });
    h * d
}

fn parse(input: &str) -> Result<Vec<(Direction, i32)>> {
    let mut result = Vec::new();
    for l in input.lines() {
        let mut splitetd = l.split_ascii_whitespace();
        let l = splitetd.next().ok_or("no line")?;
        let d = Direction::from_str(l)?;
        let num = splitetd.next().ok_or("err")?.parse::<i32>()?;
        result.push((d, num));
    }
    Ok(result)
}

enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err("cannot parse"),
        }
    }
}
