use std::collections::{BTreeMap, HashSet};

fn main() {
    println!("Hello, world!");
    let ipt = include_str!("../input/input");
    let [left_top, right_bottom] = parse(ipt);
    let solved = solve(left_top, right_bottom);
    println!("{}", solved);
    // println!("{}", (solved * (solved + 1)) / 2);
    test()
}

fn solve(left_top: Position, right_bottom: Position) -> usize {
    let (y_low, y_high) = (right_bottom.y, left_top.y);
    let v = (y_low..=y_high)
        .map(calc_v)
        .fold(BTreeMap::new(), |mut acc, cur| {
            for (k, v) in cur {
                let set = acc.entry(k).or_insert_with(HashSet::new);
                for i in v {
                    set.insert(i);
                }
            }
            acc
        });

    let (x_low, x_high) = (left_top.x, right_bottom.x);
    println!("{:?}", v);

    let mut y_x_map = BTreeMap::new();
    for (y, ts) in v {
        for t in ts {
            (x_low..=x_high).for_each(|x| {
                let x_set = calc_v_x(t, x);
                if x_set.contains(&8) && y == 5 {
                    println!("hh {} {}", t, x);
                }
                let set = y_x_map.entry(y).or_insert_with(HashSet::new);
                let new_set = set.union(&x_set).copied().collect();
                *set = new_set;
            })
        }
    }
    println!("--------------");
    println!("{:?}", y_x_map);
    println!("--------------");
    y_x_map.iter().map(|(_, v)| v.len()).sum()
}

fn parse(ipt: &str) -> [Position; 2] {
    let mut splitted = ipt.split_whitespace();
    let x = splitted.nth(2).expect("no x");
    let x = &x[..x.len() - 1];
    let y = splitted.next().expect("no y");
    let mut x = x[2..].split("..");

    let (x1, x2) = (
        x.next().unwrap().parse::<i32>().unwrap(),
        x.next().unwrap().parse::<i32>().unwrap(),
    );

    let mut y = y[2..].split("..");
    let (y1, y2) = (
        y.next().unwrap().parse::<i32>().unwrap(),
        y.next().unwrap().parse::<i32>().unwrap(),
    );
    [Position { x: x1, y: y2 }, Position { x: x2, y: y1 }]
}

fn calc_v(a: i32) -> BTreeMap<i32, HashSet<i32>> {
    let mut mp = BTreeMap::new();
    for t in 1..10000 {
        let bunsi = 2 * a + t * (t + 1) - 2 * t;
        let bunbo = 2 * t;

        if bunsi % bunbo != 0 {
            continue;
        }
        let v = bunsi / bunbo;
        mp.entry(v).or_insert_with(HashSet::new).insert(t);
    }
    mp
}

fn calc_v_x(t0: i32, a: i32) -> HashSet<i32> {
    let f = |alpha: i32| {
        let mut result = HashSet::new();
        for t in 1..=t0 {
            let bunsi = (2 * a + t * (t + 1) * alpha - 2 * alpha * t);
            let bunbo = (2 * t);
            let vx = bunsi / bunbo;
            if vx > 0 && bunsi % bunbo == 0 {
                if t0 == t {
                    if t <= vx {
                        result.insert(bunsi / bunbo);
                    }
                } else if (vx * (vx + 1) / 2) == a {
                    println!("{} {} {}", t0, a, vx);
                    result.insert(bunsi / bunbo);
                }
            }
        }
        result
    };
    let x0 = f(1);
    x0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn next(self) -> Self {
        let next_x = if self.x > 0 { self.x - 1 } else { self.x + 1 };
        Velocity {
            x: next_x,
            y: self.y - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn next(self, velocity: &Velocity) -> Self {
        Position {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
        }
    }
}

fn step(pos: Position, vel: Velocity) -> (Position, Velocity) {
    (pos.next(&vel), vel.next())
}

fn test() {
    let ipt = r#"23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
8,-2    27,-8   30,-5   24,-7"#;

    let mut map = BTreeMap::new();
    for l in ipt.lines() {
        for sp in l.split_whitespace() {
            if let [x, y] = sp
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                map.entry(y).or_insert_with(HashSet::new).insert(x);
            }
        }
    }

    println!("--------------");
    println!("{:?}", map);
    println!("--------------");
}
