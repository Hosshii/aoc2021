use std::str::FromStr;

fn main() {
    let ipt = include_str!("../input/input");
    let parsed = parse(ipt);
    let v = solve2(parsed);
    dbg!(v);
}

fn solve2(ops: Vec<Operation>) -> u128 {
    let v = ops.into_iter().fold(Vec::<Cube>::new(), |acc, cur| {
        let mut result = Vec::new();
        for cube in acc {
            result.append(&mut cube.split(&cur.cube));
        }
        match cur.kind {
            OpKind::TurnOn => {
                result.push(cur.cube);
            }
            OpKind::TurnOff => {}
        }
        result
    });

    v.iter().map(|c| c.volume()).sum::<u128>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    base: Position,
    x: u128,
    y: u128,
    z: u128,
}

impl Cube {
    fn volume(&self) -> u128 {
        self.x * self.y * self.z
    }

    fn x_start(&self) -> i128 {
        self.base.x
    }

    fn x_end(&self) -> i128 {
        self.base.x + self.x as i128
    }

    fn y_start(&self) -> i128 {
        self.base.y
    }

    fn y_end(&self) -> i128 {
        self.base.y + self.y as i128
    }

    fn z_start(&self) -> i128 {
        self.base.z
    }

    fn z_end(&self) -> i128 {
        self.base.z + self.z as i128
    }

    fn is_overlap(&self, rhs: &Self) -> bool {
        let x_overlap =
            self.base.x + self.x as i128 >= rhs.base.x && rhs.base.x + rhs.x as i128 >= self.base.x;
        let y_overlap =
            self.base.y + self.y as i128 >= rhs.base.y && rhs.base.y + rhs.y as i128 >= self.base.y;
        let z_overlap =
            self.base.z + self.z as i128 >= rhs.base.z && rhs.base.z + rhs.z as i128 >= self.base.z;
        x_overlap && y_overlap && z_overlap
    }

    fn split(self, rhs: &Self) -> Vec<Cube> {
        fn _split(
            c: Cube,
            rhs: &Cube,
            pos: i128,
            f: &dyn Fn(Cube, i128) -> Vec<Cube>,
        ) -> Vec<Cube> {
            if c.is_overlap(rhs) {
                f(c, pos)
            } else {
                vec![c]
            }
        }

        let (x1, x2) = (rhs.x_start(), rhs.x_end());
        let cubes = self.split_x(x1);

        let cubes = cubes
            .into_iter()
            .map(|c| _split(c, rhs, x2, &Cube::split_x))
            .flatten();

        let (y1, y2) = (rhs.y_start(), rhs.y_end());
        let cubes = cubes
            .into_iter()
            .map(|c| _split(c, rhs, y1, &Cube::split_y))
            .flatten()
            .map(|c| _split(c, rhs, y2, &Cube::split_y))
            .flatten();

        let (z1, z2) = (rhs.z_start(), rhs.z_end());
        let cubes = cubes
            .into_iter()
            .map(|c| _split(c, rhs, z1, &Cube::split_z))
            .flatten()
            .map(|c| _split(c, rhs, z2, &Cube::split_z))
            .flatten();

        let cubes = cubes.into_iter().filter(|c| !rhs.contains(c)).collect();

        cubes
    }

    fn split_x(self, x: i128) -> Vec<Self> {
        if x <= self.x_start() || self.x_end() <= x {
            return vec![self];
        }

        let lhs = Cube {
            base: self.base,
            x: (x - self.x_start()) as u128,
            ..self
        };

        let rhs = Cube {
            base: Position {
                x: x as i128,
                ..self.base
            },
            x: (self.x_end() - x) as u128,
            ..self
        };

        vec![lhs, rhs]
    }

    fn split_y(self, y: i128) -> Vec<Self> {
        if y <= self.y_start() || self.y_end() <= y {
            return vec![self];
        }

        let lhs = Cube {
            base: self.base,
            y: (y - self.y_start()) as u128,
            ..self
        };

        let rhs = Cube {
            base: Position {
                y: y as i128,
                ..self.base
            },
            y: (self.y_end() - y) as u128,
            ..self
        };

        vec![lhs, rhs]
    }

    fn split_z(self, z: i128) -> Vec<Self> {
        if z <= self.z_start() || (self.z_end() as i128) <= z {
            return vec![self];
        }

        let lhs = Cube {
            base: self.base,
            z: (z - self.z_start()) as u128,
            ..self
        };

        let rhs = Cube {
            base: Position {
                z: z as i128,
                ..self.base
            },
            z: (self.z_end() as i128 - z) as u128,
            ..self
        };

        vec![lhs, rhs]
    }

    fn contains(&self, rhs: &Self) -> bool {
        let x_contains =
            self.base.x <= rhs.base.x && rhs.base.x + rhs.x as i128 <= self.base.x + self.x as i128;
        let y_contains =
            self.base.y <= rhs.base.y && rhs.base.y + rhs.y as i128 <= self.base.y + self.y as i128;
        let z_contains =
            self.base.z <= rhs.base.z && rhs.base.z + rhs.z as i128 <= self.base.z + self.z as i128;
        x_contains && y_contains && z_contains
    }
}

fn operation(mut cubes: Vec<Cube>, op: &Operation) {
    match op.kind {
        OpKind::TurnOn => {}
        OpKind::TurnOff => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i128,
    y: i128,
    z: i128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Operation {
    cube: Cube,
    kind: OpKind,
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
                        iter.next().unwrap()[2..].parse::<i128>().unwrap(),
                        iter.next().unwrap().parse::<i128>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let base = Position {
                x: v[0].0,
                y: v[1].0,
                z: v[2].0,
            };
            let x = (v[0].1 - v[0].0 + 1) as u128;
            let y = (v[1].1 - v[1].0 + 1) as u128;
            let z = (v[2].1 - v[2].0 + 1) as u128;
            let cube = Cube { base, x, y, z };

            Operation { cube, kind: op }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let cube = Cube {
            base: Position { x: 0, y: 0, z: 0 },
            x: 3,
            y: 3,
            z: 3,
        };

        let rhs = Cube {
            base: Position { x: 1, y: 0, z: 0 },
            x: 3,
            y: 3,
            z: 3,
        };

        let expected = Cube {
            base: Position { x: 0, y: 0, z: 0 },
            x: 1,
            y: 3,
            z: 3,
        };

        let expected = vec![expected];

        let splitted = cube.split(&rhs);

        assert_eq!(expected, splitted);
    }

    #[test]
    fn test_split_x() {
        let cube = Cube {
            base: Position { x: 0, y: 0, z: 0 },
            x: 3,
            y: 3,
            z: 3,
        };

        let expected = vec![
            Cube {
                base: Position { x: 0, y: 0, z: 0 },
                x: 1,
                y: 3,
                z: 3,
            },
            Cube {
                base: Position { x: 1, y: 0, z: 0 },
                x: 2,
                y: 3,
                z: 3,
            },
        ];
        let splitted = cube.split_x(1);

        assert_eq!(splitted, expected);
    }

    #[test]
    fn test_split_y() {
        let cube = Cube {
            base: Position { x: 0, y: 0, z: 0 },
            x: 3,
            y: 3,
            z: 3,
        };

        let expected = vec![
            Cube {
                base: Position { x: 0, y: 0, z: 0 },
                x: 3,
                y: 1,
                z: 3,
            },
            Cube {
                base: Position { x: 0, y: 1, z: 0 },
                x: 3,
                y: 2,
                z: 3,
            },
        ];
        let splitted = cube.split_y(1);

        assert_eq!(expected, splitted);
    }

    #[test]
    fn test_split_z() {
        let cube = Cube {
            base: Position { x: 0, y: 0, z: 0 },
            x: 3,
            y: 3,
            z: 3,
        };

        let expected = vec![
            Cube {
                base: Position { x: 0, y: 0, z: 0 },
                x: 3,
                y: 3,
                z: 1,
            },
            Cube {
                base: Position { x: 0, y: 0, z: 1 },
                x: 3,
                y: 3,
                z: 2,
            },
        ];

        let splitted = cube.split_z(1);

        assert_eq!(expected, splitted);
    }
}
