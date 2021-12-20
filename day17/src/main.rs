use std::cmp;

fn main() {
    println!("Hello, world!");
    let ipt = include_str!("../input/input");
    let [left_top, right_bottom] = parse(ipt);
    let solved = solve((left_top.y, right_bottom.y));
    println!("{}", solved);
    println!("{}", (solved * (solved + 1)) / 2);
}

fn solve((high, low): (i32, i32)) -> i32 {
    println!("{} {}", high, low);
    (low..high).map(calc_v).max().unwrap()
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

fn calc_v(a: i32) -> i32 {
    let mut v = 0;
    for t in 1..10000 {
        let bunsi = 2 * a + t * (t + 1);
        let bunbo = 2 * (t + 1);

        if bunsi % bunbo != 0 {
            continue;
        }
        v = v.max(bunsi / bunbo);
    }
    v
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
