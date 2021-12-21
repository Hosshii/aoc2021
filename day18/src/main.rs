use std::{fmt::Display, ops::Add};

fn main() -> MyResult<()> {
    let inputs = include_str!("../input/input");
    // for l in inputs.lines() {
    //     let (remain, mut parsed) = parse_pair(l)?;
    //     let parsed_display = DisplayPair(&parsed);
    //     println!("{}", parsed_display.display());

    //     loop {
    //         let changed = solve(&mut parsed, 0).2;

    //         let parsed_display = DisplayPair(&parsed);
    //         println!("{}", parsed_display.display());
    //         println!();
    //         if !changed {
    //             break;
    //         }
    //     }
    //     println!();
    // }

    let a = inputs
        .lines()
        .map(|l| parse_pair(l).unwrap().1)
        .collect::<Vec<_>>();

    let mut result = 0;
    for l in a.clone() {
        for r in a.clone() {
            if l == r {
                continue;
            }
            let mut added = l.clone() + r;
            _solve(&mut added);
            result = result.max(added.magnitude());
        }
    }

    println!("{:?}", result);
    Ok(())
}

fn display_pair(p: &Pair<u8>) {
    let parsed_display = DisplayPair(&p);
    println!("{}", parsed_display.display());
}

fn _solve(p: &mut Pair<u8>) {
    while explode(p, 0).2 || split(p, 0) {}
}

// depth is 0 indexed
fn explode(p: &mut Pair<u8>, depth: u8) -> (Option<u8>, Option<u8>, bool) {
    let (ref mut lhs, ref mut rhs) = p.pair;
    if depth >= 3 && lhs.can_explode() {
        let (lhs_v, rhs_v) = lhs.explode().unwrap();
        rhs.add(rhs_v, true);
        return (Some(lhs_v), None, true);
    }
    if depth >= 3 && rhs.can_explode() {
        let (lhs_v, rhs_v) = rhs.explode().unwrap();
        lhs.add(lhs_v, false);
        return (None, Some(rhs_v), true);
    }

    if let PairKind::SubPair(ref mut sub) = lhs {
        let (lhs_v, rhs_v, mut changed) = explode(&mut *sub, depth + 1);

        if let Some(rhs_v) = rhs_v {
            rhs.add(rhs_v, true);

            changed = true;
        }

        if changed {
            return (lhs_v, None, changed);
        }
    }

    if let PairKind::SubPair(ref mut sub) = rhs {
        let (lhs_v, rhs_v, mut changed) = explode(&mut *sub, depth + 1);
        if let Some(lhs_v) = lhs_v {
            lhs.add(lhs_v, false);
            changed = true;
        }
        return (None, rhs_v, changed);
    }

    (None, None, false)
}

fn split(p: &mut Pair<u8>, depth: u8) -> bool {
    let (ref mut lhs, ref mut rhs) = p.pair;
    if lhs.can_split() {
        lhs.split();
        return true;
    }

    if let PairKind::SubPair(ref mut sub) = lhs {
        if split(&mut *sub, depth + 1) {
            return true;
        }
    }

    if rhs.can_split() {
        rhs.split();
        return true;
    }

    if let PairKind::SubPair(ref mut sub) = rhs {
        return split(&mut *sub, depth + 1);
    }

    false
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair<T> {
    pair: (PairKind<T>, PairKind<T>),
}

impl Pair<u8> {
    fn magnitude(&self) -> u32 {
        3 * self.pair.0.magnitude() + 2 * self.pair.1.magnitude()
    }
}

impl<T> Add for Pair<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pair {
            pair: (
                PairKind::SubPair(Box::new(self)),
                PairKind::SubPair(Box::new(rhs)),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PairKind<T> {
    Literal(T),
    SubPair(Box<Pair<T>>),
}

impl PairKind<u8> {
    fn explode(&mut self) -> Option<(u8, u8)> {
        if !self.can_explode() {
            None
        } else {
            let old = std::mem::replace(self, PairKind::Literal(0));
            match old {
                PairKind::SubPair(sub) => {
                    let (lhs, rhs) = sub.pair;
                    if let (PairKind::Literal(lhs), PairKind::Literal(rhs)) = (lhs, rhs) {
                        Some((lhs, rhs))
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn split(&mut self) {
        if self.can_split() {
            let next_pair = match &self {
                PairKind::Literal(x) => {
                    let (lhs, rhs) = ((*x as f64 / 2.0).floor(), (*x as f64 / 2.0).ceil());
                    PairKind::SubPair(Box::new(Pair {
                        pair: (PairKind::Literal(lhs as u8), PairKind::Literal(rhs as u8)),
                    }))
                }
                _ => unreachable!(),
            };
            *self = next_pair;
        }
    }

    fn can_explode(&self) -> bool {
        match self {
            PairKind::Literal(_) => false,
            PairKind::SubPair(pair) => {
                let (lhs, rhs) = &pair.pair;
                matches!((lhs, rhs), (PairKind::Literal(_), PairKind::Literal(_)))
                // if let (PairKind::Literal(_), PairKind::Literal(_)) = (lhs, rhs) {
                //     true
                // } else {
                //     false
                // }
            }
        }
    }

    fn can_split(&self) -> bool {
        match self {
            PairKind::Literal(x) => *x >= 10,
            PairKind::SubPair(_) => false,
        }
    }

    // left: true, right: false
    fn add(&mut self, x: u8, left: bool) {
        match self {
            PairKind::Literal(ref mut y) => {
                *y += x;
            }
            PairKind::SubPair(ref mut pair) => {
                let (lhs, rhs) = &mut pair.pair;
                if left {
                    lhs.add(x, left);
                } else {
                    rhs.add(x, left);
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            PairKind::Literal(x) => *x as u32,
            PairKind::SubPair(pair) => pair.magnitude() as u32,
        }
    }
}

struct DisplayPair<'a, T>(&'a Pair<T>);

impl<'a, T> DisplayPair<'a, T>
where
    T: Display,
{
    fn display(&self) -> String {
        let Pair { pair: (lhs, rhs) } = self.0;
        let (lhs, rhs) = (DisplayPairKind(lhs), DisplayPairKind(rhs));
        format!("[{},{}]", lhs.display(), rhs.display())
    }
}

struct DisplayPairKind<'a, T>(&'a PairKind<T>);

impl<'a, T> DisplayPairKind<'a, T>
where
    T: Display,
{
    fn display(&self) -> String {
        let kind = self.0;
        match kind {
            PairKind::Literal(t) => t.to_string(),
            PairKind::SubPair(p) => {
                let p = DisplayPair(p);
                p.display()
            }
        }
    }
}

fn parse_pair(ipt: &str) -> MyResult<(&str, Pair<u8>)> {
    let mut iter = ipt.char_indices().peekable();
    if iter.next().expect("expect '['").1 != '[' {
        return Err(format!("expect '[', found: {}", ipt).into());
    }
    let (idx, _) = iter.peek().expect("expect char");
    let (next, lhs) = parse_elem(&ipt[*idx..])?;
    let mut iter = next.char_indices().peekable();
    iter.next().expect("expect ','");

    let (idx, _) = iter.peek().expect("expect char");
    let (next, rhs) = parse_elem(&next[*idx..])?;

    let mut iter = next.char_indices().peekable();
    iter.next().expect("expect ']'");
    let next_input = if let Some((idx, _)) = iter.peek() {
        &next[*idx..]
    } else {
        ""
    };
    Ok((next_input, Pair { pair: (lhs, rhs) }))
}

fn parse_elem(ipt: &str) -> MyResult<(&str, PairKind<u8>)> {
    let mut iter = ipt.char_indices().peekable();
    let (_, c) = iter.peek().expect("expect char");
    match c.to_digit(10) {
        Some(digit) => {
            iter.next().unwrap();
            let next_idx = iter.peek().expect("expect char").0;
            let next_input = &ipt[next_idx..];
            Ok((next_input, PairKind::Literal(digit as u8)))
        }
        None => {
            let next_idx = iter.peek().expect("expect char").0;
            let (next_ipt, rhs) = parse_pair(&ipt[next_idx..])?;
            Ok((next_ipt, PairKind::SubPair(Box::new(rhs))))
        }
    }
}
