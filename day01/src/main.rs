use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let f = File::open("input")?;
    let bufreader = BufReader::new(f);
    let bufreader = collect_data(bufreader)?;

    let out = solve(bufreader.as_bytes())?;
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

fn collect_data<T: BufRead>(ipt: T) -> Result<String> {
    let mut buf = String::new();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for (i, l) in ipt.lines().enumerate() {
        let cur = l?.parse::<usize>()?;

        if i == 0 {
            a += cur;
        } else if i == 1 {
            a += cur;
            b += cur;
        } else {
            a += cur;
            b += cur;
            c += cur;
            if i % 3 == 2 {
                buf.push_str(&a.to_string());
                a = 0;
            } else if i % 3 == 0 {
                buf.push_str(&b.to_string());
                b = 0;
            } else {
                buf.push_str(&c.to_string());
                c = 0;
            }
            buf.push('\n');
        }
    }
    Ok(buf)
}
