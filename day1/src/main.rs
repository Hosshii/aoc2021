use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let f = File::open("input")?;
    let bufreader = BufReader::new(f);
    let out = solve(bufreader)?;
    println!("{}", out);
    Ok(())
}

fn solve<T: BufRead>(ipt: T) -> Result<usize> {
    let mut before = None;
    let mut sum = 0;
    for l in ipt.lines() {
        let cur = l?.parse::<usize>()?;
        if let Some(before) = before {
            if cur > before {
                sum += 1;
            }
        }
        before = Some(cur);
    }
    Ok(sum)
}
