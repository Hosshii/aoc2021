use std::{ops::RangeInclusive, str::FromStr};

fn main() {
    let ipt = include_str!("../input/input");
    let parsed = parse(ipt);
    let cubes = new_cubes();
    let solved = solve1(cubes, &parsed);
    dbg!(solved);
}

fn solve1(mut cubes: Cubes, ops: &[Operation]) -> usize {
    for op in ops {
        for x in op.ranges.x.clone() {
            if x < -50 || 50 < x {
                continue;
            }
            for y in op.ranges.y.clone() {
                if y < -50 || 50 < y {
                    continue;
                }
                for z in op.ranges.z.clone() {
                    if z < -50 || 50 < z {
                        continue;
                    }
                    let (x, y, z) = ((x + 50) as usize, (y + 50) as usize, (z + 50) as usize);
                    match op.action {
                        OpKind::TurnOn => cubes[x][y][z] = true,
                        OpKind::TurnOff => cubes[x][y][z] = false,
                    }
                }
            }
        }
    }

    cubes
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.iter().filter(|z| **z).count())
                .sum::<usize>()
        })
        .sum::<usize>()
}

type Cubes = Vec<Vec<Vec<bool>>>;

struct Position {
    x: usize,
    y: usize,
    z: usize,
}

fn turn_on(cubes: &mut Cubes, position: &Position) {
    cubes[position.x][position.y][position.z] = true;
}

fn turn_off(cubes: &mut Cubes, position: &Position) {
    cubes[position.x][position.y][position.z] = false;
}

fn new_cubes() -> Cubes {
    vec![vec![vec![false; 101]; 101]; 101]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ranges {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Operation {
    ranges: Ranges,
    action: OpKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OpKind {
    TurnOn,
    TurnOff,
}

impl FromStr for OpKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OpKind::TurnOn),
            "off" => Ok(OpKind::TurnOff),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let (op, xyz) = (iter.next().unwrap(), iter.next().unwrap());

            let op = OpKind::from_str(op).unwrap();

            let mut xyz = xyz.split(',');
            let xyz = [
                xyz.next().unwrap(),
                xyz.next().unwrap(),
                xyz.next().unwrap(),
            ];
            let v = xyz
                .iter()
                .map(|v| {
                    let mut iter = v.split("..");
                    (
                        iter.next().unwrap()[2..].parse::<isize>().unwrap(),
                        iter.next().unwrap().parse::<isize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let ranges = Ranges {
                x: v[0].0..=v[0].1,
                y: v[1].0..=v[1].1,
                z: v[2].0..=v[2].1,
            };
            Operation { ranges, action: op }
        })
        .collect()
}
