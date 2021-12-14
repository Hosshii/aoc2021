fn main() {
    let ipt = include_str!("../input/input");
    let (points, folds) = parse(ipt);
    let paper = make_paper(points);
    // println!("{:?}", paper);

    let folded = folds.iter().fold(paper, |acc, cur| fold(acc, &cur));
    for row in &folded {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{:?}", count(&folded));
}

type Paper = Vec<Vec<bool>>;

fn count(p: &Paper) -> usize {
    p.iter().fold(0, |acc, now| {
        acc + now.iter().fold(0, |acc, &x| acc + if x { 1 } else { 0 })
    })
}

fn fold(mut paper: Paper, fold: &Fold) -> Paper {
    match fold {
        &Fold::Y(y) => {
            for i in 0..y as usize {
                for x in 0..paper[i].len() {
                    paper[i][x] |= paper[y as usize * 2 - i][x];
                }
            }
            paper.split_off(y as usize + 1);
            paper
        }

        &Fold::X(line) => {
            for x in 0..line as usize {
                for y in 0..paper.len() {
                    paper[y][x] |= paper[y][line as usize * 2 - x];
                }
            }
            for y in 0..paper.len() {
                paper[y].split_off(line as usize + 1);
            }
            paper
        }
    }
}

fn make_paper(point: Vec<Point>) -> Paper {
    let (m_x, m_y) = point
        .iter()
        .fold((0, 0), |(mx, my), &Point { x, y }| (x.max(mx), y.max(my)));
    let mut paper = vec![vec![false; m_x as usize + 1]; m_y as usize + 1];
    for Point { x, y } in point {
        paper[y as usize][x as usize] = true;
    }
    paper
}

fn parse(ipt: &str) -> (Vec<Point>, Vec<Fold>) {
    let points = ipt
        .lines()
        .take_while(|line| line != &"")
        .map(|line| {
            let mut iter = line.split(',');
            let lhs = iter.next().expect("no lhs");
            let rhs = iter.next().expect("no rhs");
            let x = lhs.parse::<u32>().expect("not an int");
            let y = rhs.parse::<u32>().expect("not an int");
            Point::new(x, y)
        })
        .collect::<Vec<_>>();

    let folds = ipt
        .lines()
        .skip_while(|line| line != &"")
        .skip(1)
        .map(|line| {
            let mut pos = line
                .split_ascii_whitespace()
                .nth(2)
                .expect("no position")
                .split('=');
            let (kind, pos) = (
                pos.next().expect("no kind"),
                pos.next().expect("no position"),
            );
            match kind {
                "y" => Fold::Y(pos.parse::<u32>().expect("not an int")),
                "x" => Fold::X(pos.parse::<u32>().expect("not an int")),
                _ => panic!("unknown kind"),
            }
        })
        .collect::<Vec<_>>();

    (points, folds)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u32),
    Y(u32),
}
