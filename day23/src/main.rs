use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut, Range},
};

fn main() {
    println!("Hello, world!");
    let ipt = include_str!("../input/input");
    let (hole, room) = parse::<11, 2, 4>(ipt);
    dbg!(&hole);
    dbg!(&room);
    let solved = solve(hole, room);
    dbg!(solved);
}

fn solve<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    hole: Hole<H>,
    room: SideRoom<S_DEPTH, S_NUM>,
) -> u64 {
    let mut cache = HashMap::new();
    let end_hole = [NodeKind::Space; H];
    let end_hole = Hole(end_hole);
    let mut end_room = [[NodeKind::Space; S_DEPTH]; S_NUM];
    for (i, room) in (&mut end_room).iter_mut().enumerate() {
        let a = (b'A' + i as u8) as char;
        let kind = AmphipodKind::from_char(a).unwrap();
        let kind = NodeKind::Amphipod(kind);
        *room = [kind; S_DEPTH];
    }
    let end_room = SideRoom(end_room);

    cache.insert((end_hole, end_room), 0);

    _solve(hole, room, &mut cache).unwrap()
}

fn _solve<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    hole: Hole<H>,
    room: SideRoom<S_DEPTH, S_NUM>,
    cache: &mut HashMap<(Hole<H>, SideRoom<S_DEPTH, S_NUM>), u64>,
) -> Option<u64> {
    // println!();
    // println!("{}", hole);
    // println!("{}", room);
    if cache.get(&(hole, room)).is_some() {
        return cache.get(&(hole, room)).cloned();
    }

    let mut costs = Vec::new();

    // room to hole
    for hole_idx in 0..hole.0.len() {
        for room_idx in 0..room.0.len() {
            let depth = room.0[room_idx]
                .iter()
                .filter(|v| v == &&NodeKind::Space)
                .count();
            if depth == S_DEPTH {
                continue;
            }

            let mut next_room = room.clone();
            let mut next_hole = hole.clone();
            if let Some(move_cost) =
                room_to_hole((room_idx, depth), hole_idx, &mut next_hole, &mut next_room)
            {
                let cost = _solve(next_hole, next_room, cache);
                if let Some(min_cost) = cost {
                    costs.push(min_cost + move_cost);
                }
            }
        }
    }

    // hole to room
    for hole_idx in 0..hole.0.len() {
        for room_idx in 0..room.0.len() {
            let depth = room.0[room_idx]
                .iter()
                .filter(|v| v == &&NodeKind::Space)
                .count();
            if depth == 0 {
                continue;
            }
            let depth = depth - 1;

            let mut next_room = room.clone();
            let mut next_hole = hole.clone();
            if let Some(move_cost) =
                hole_to_room(hole_idx, (room_idx, depth), &mut next_hole, &mut next_room)
            {
                let cost = _solve(next_hole, next_room, cache);
                if let Some(min_cost) = cost {
                    costs.push(min_cost + move_cost);
                }
            }
        }
    }

    let min_cost = costs.into_iter().min();
    if let Some(min_cost) = min_cost {
        cache.insert((hole, room), min_cost);
    }
    min_cost
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hole<const H: usize>([NodeKind; H]);

impl<const N: usize> Display for Hole<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.0.iter() {
            write!(f, "{}", v.as_char())?;
        }
        Ok(())
    }
}

impl<const H: usize> Index<usize> for Hole<H> {
    type Output = NodeKind;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const H: usize> IndexMut<usize> for Hole<H> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const H: usize> Index<Range<usize>> for Hole<H> {
    type Output = [NodeKind];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SideRoom<const DEPTH: usize, const NUM: usize>([[NodeKind; DEPTH]; NUM]);
impl<const DEPTH: usize, const NUM: usize> Display for SideRoom<DEPTH, NUM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DEPTH {
            write!(f, "##")?;
            for x in 0..NUM {
                write!(f, "{}#", self.0[x][y].as_char())?;
            }
            writeln!(f, "#")?;
        }
        Ok(())
    }
}

impl<const DEPTH: usize, const NUM: usize> SideRoom<DEPTH, NUM> {
    fn can_enter(&self, amphipod: &AmphipodKind) -> bool {
        let x = *amphipod as usize;
        let room = &self.0[x];
        let kind = NodeKind::Amphipod(*amphipod);
        room.iter().all(|&k| k == kind || k == NodeKind::Space)
    }

    fn can_exit(&self, id: usize, depth: usize) -> bool {
        let room = &self.0[id];
        room.iter().take(depth).all(|&k| k == NodeKind::Space)
    }
}

impl<const DEPTH: usize, const NUM: usize> Index<usize> for SideRoom<DEPTH, NUM> {
    type Output = [NodeKind];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const DEPTH: usize, const NUM: usize> IndexMut<usize> for SideRoom<DEPTH, NUM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn parse<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    input: &str,
) -> (Hole<H>, SideRoom<S_DEPTH, S_NUM>) {
    let hall = [NodeKind::Space; H];
    let mut lines = input.lines();
    lines.next().unwrap();
    lines.next().unwrap();
    let row1 = lines
        .take(S_DEPTH)
        .map(|line| {
            line.chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| NodeKind::Amphipod(AmphipodKind::from_char(c).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = [[NodeKind::Space; S_DEPTH]; S_NUM];
    for y in 0..S_DEPTH {
        for x in 0..S_NUM {
            result[x][y] = row1[y][x];
        }
    }
    (Hole(hall), SideRoom(result))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeKind {
    Space,
    Amphipod(AmphipodKind),
}

impl NodeKind {
    fn from_char(c: char) -> NodeKind {
        match c {
            '.' => NodeKind::Space,
            c => NodeKind::Amphipod(AmphipodKind::from_char(c).unwrap()),
        }
    }

    fn as_char(&self) -> char {
        match self {
            NodeKind::Space => '.',
            NodeKind::Amphipod(amphipod) => amphipod.as_char(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AmphipodKind {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl AmphipodKind {
    fn cost(&self) -> u64 {
        match self {
            AmphipodKind::A => 1,
            AmphipodKind::B => 10,
            AmphipodKind::C => 100,
            AmphipodKind::D => 1000,
        }
    }
    fn from_char(c: char) -> Option<AmphipodKind> {
        match c {
            'A' => Some(AmphipodKind::A),
            'B' => Some(AmphipodKind::B),
            'C' => Some(AmphipodKind::C),
            'D' => Some(AmphipodKind::D),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            AmphipodKind::A => 'A',
            AmphipodKind::B => 'B',
            AmphipodKind::C => 'C',
            AmphipodKind::D => 'D',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Idx {
    Hole(usize),
    SideRoom(usize, usize), // lhs: side room number, rhs: side room depth
}

fn hole_to_room<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    from: usize,
    to: (usize, usize),
    hole: &mut Hole<H>,
    side_room: &mut SideRoom<S_DEPTH, S_NUM>,
) -> Option<u64> {
    if !can_move(
        &Idx::Hole(from),
        &Idx::SideRoom(to.0, to.1),
        hole,
        side_room,
    ) {
        None
    } else {
        match hole[from] {
            NodeKind::Amphipod(amphipod) => {
                let cost = amphipod.cost();
                let hole_distance =
                    (side_room_idx_to_hole_idx(to.0) as isize - from as isize).abs() as u64;
                let room_distance = to.1 as u64 + 1;
                let cost = cost * (hole_distance + room_distance);
                std::mem::swap(&mut hole[from], &mut side_room[to.0][to.1]);
                Some(cost)
            }
            _ => unreachable!(),
        }
    }
}

fn room_to_hole<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    from: (usize, usize),
    to: usize,
    hole: &mut Hole<H>,
    side_room: &mut SideRoom<S_DEPTH, S_NUM>,
) -> Option<u64> {
    if !can_move(
        &Idx::SideRoom(from.0, from.1),
        &Idx::Hole(to),
        hole,
        side_room,
    ) {
        None
    } else {
        match side_room[from.0][from.1] {
            NodeKind::Amphipod(amphipod) => {
                let cost = amphipod.cost();
                let hole_distance =
                    (side_room_idx_to_hole_idx(from.0) as isize - to as isize).abs() as u64 + 1;
                let cost = cost * (from.1 as u64 + hole_distance);
                std::mem::swap(&mut side_room[from.0][from.1], &mut hole[to]);
                Some(cost)
            }
            _ => unreachable!(),
        }
    }
}

fn can_move<const H: usize, const S_DEPTH: usize, const S_NUM: usize>(
    from: &Idx,
    to: &Idx,
    hole: &Hole<H>,
    side_room: &SideRoom<S_DEPTH, S_NUM>,
) -> bool {
    match (from, to) {
        (&Idx::Hole(hole_idx), &Idx::SideRoom(room_id, depth)) => {
            let kind = hole[hole_idx];
            let kind = if let NodeKind::Amphipod(amphipod) = kind {
                amphipod
            } else {
                return false;
            };

            // 自分と違う部屋には入らない
            if kind as usize != room_id {
                return false;
            }

            // holeに障害がないか
            let hole_idx2 = side_room_idx_to_hole_idx(room_id);
            let (lhs, rhs) = if hole_idx < hole_idx2 {
                (hole_idx + 1, hole_idx2)
            } else {
                (hole_idx2, hole_idx - 1)
            };
            let can_move_hole = hole[lhs..(rhs + 1)].iter().all(|&k| k == NodeKind::Space);

            let can_move_side_room = side_room.can_enter(&kind);
            let expect_depth = side_room[room_id]
                .iter()
                .filter(|v| v == &&NodeKind::Space)
                .count();

            // 全部埋まってたら入れない
            if expect_depth == 0 {
                return false;
            }
            let expect_depth = expect_depth - 1;

            can_move_hole && can_move_side_room && expect_depth == depth
        }
        (&Idx::SideRoom(room_id, depth), &Idx::Hole(hole_idx)) => {
            // room の上にはいけない
            match hole_idx {
                2 | 4 | 6 | 8 => return false,
                _ => {}
            }
            // dbg!(from, to);
            let node = side_room[room_id][depth];
            let kind = if let NodeKind::Amphipod(amphipod) = node {
                amphipod
            } else {
                return false;
            };

            // すでに正しいところに入ってたらそこから出ない
            if let NodeKind::Amphipod(x) = side_room[room_id][depth] {
                if x as usize == room_id
                    && side_room[room_id][depth..]
                        .iter()
                        .all(|v| *v == side_room[room_id][depth])
                {
                    return false;
                }
            }

            let can_exit_room = side_room.can_exit(room_id, depth);
            let hole_idx2 = side_room_idx_to_hole_idx(room_id);

            let (lhs, rhs) = if hole_idx < hole_idx2 {
                (hole_idx, hole_idx2)
            } else {
                (hole_idx2, hole_idx)
            };

            let can_move_hole = hole[lhs..(rhs + 1)].iter().all(|&k| k == NodeKind::Space);

            can_exit_room && can_move_hole
        }
        //hole -> holeは動かない
        _ => false,
    }
}

fn side_room_idx_to_hole_idx(side_room_id: usize) -> usize {
    (side_room_id + 1) * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_move() {
        let hole = [NodeKind::Space; 11];
        let hole = Hole(hole);
        let side_room = [
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
        ];
        let side_room = SideRoom(side_room);

        let from = Idx::Hole(0);
        let to = Idx::SideRoom(0, 0);

        assert!(!can_move(&from, &to, &hole, &side_room));
        assert!(can_move(&to, &from, &hole, &side_room));

        // =========
        let mut hole = [NodeKind::Space; 11];

        hole[0] = NodeKind::Amphipod(AmphipodKind::A);
        hole[1] = NodeKind::Amphipod(AmphipodKind::A);

        let hole = Hole(hole);
        let side_room = [
            [NodeKind::Space, NodeKind::Space],
            [NodeKind::Space, NodeKind::Space],
        ];
        let side_room = SideRoom(side_room);

        let from = Idx::Hole(0);
        let to = Idx::SideRoom(0, 0);

        assert!(!can_move(&from, &to, &hole, &side_room));
        assert!(!can_move(&to, &from, &hole, &side_room));

        // =========
        let mut hole = [NodeKind::Space; 11];
        hole[0] = NodeKind::Amphipod(AmphipodKind::A);
        let hole = Hole(hole);
        let side_room = [[NodeKind::Space, NodeKind::Amphipod(AmphipodKind::A)]];
        let side_room = SideRoom(side_room);

        let from = Idx::Hole(0);
        let to = Idx::SideRoom(0, 0);

        assert!(!can_move(&from, &to, &hole, &side_room));
        assert!(!can_move(&to, &from, &hole, &side_room));

        //====
        let side_room = [
            [
                NodeKind::Amphipod(AmphipodKind::B),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
            [
                NodeKind::Amphipod(AmphipodKind::A),
                NodeKind::Amphipod(AmphipodKind::B),
            ],
        ];
        let side_room = SideRoom(side_room);

        let from = Idx::Hole(10);
        let to = Idx::SideRoom(0, 0);

        assert!(!can_move(&from, &to, &hole, &side_room));
        assert!(can_move(&to, &from, &hole, &side_room));
    }
}
