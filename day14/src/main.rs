use std::collections::{BTreeMap, HashMap, LinkedList};

fn main() {
    let ipt = include_str!("../input/input");
    let (polymer, def) = parse(ipt);
    // println!("{:?} {:?}", polymer, def);
    let result = (0..40).fold(polymer, |acc, _| insert(LinkedList::new(), acc, &def));
    // println!("{:?}", result);
    let result = count(&result);
    println!("{}", result);
}

fn count(list: &LinkedList<char>) -> usize {
    let tree = list.iter().fold(BTreeMap::new(), |mut acc, cur| {
        acc.entry(*cur).and_modify(|x| *x += 1).or_insert(1);
        acc
    });
    let max = tree.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = tree.iter().min_by_key(|(_, v)| *v).unwrap();
    max.1 - min.1
}

fn parse(ipt: &str) -> (LinkedList<char>, HashMap<(char, char), char>) {
    let mut lines = ipt.lines();
    let template = lines.next().unwrap().chars().collect::<LinkedList<char>>();
    lines.next().expect("No reaction lines");

    let mp = lines
        .map(|line| {
            let mut line = line.split_ascii_whitespace();
            let (lhs, rhs) = (line.next().expect("No lhs"), line.nth(1).expect("No rhs"));
            (
                (lhs.chars().next().unwrap(), lhs.chars().nth(1).unwrap()),
                rhs.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
    (template, mp)
}

fn insert(
    mut heads: LinkedList<char>,
    mut tails: LinkedList<char>,
    mp: &HashMap<(char, char), char>,
) -> LinkedList<char> {
    if tails.is_empty() {
        heads
    } else if tails.len() == 1 {
        heads.append(&mut tails);
        heads
    } else {
        let x = tails.pop_front().unwrap();
        let y = tails.front().unwrap();
        heads.push_back(x);
        if let Some(insertion) = mp.get(&(x, *y)) {
            heads.push_back(*insertion);
        }
        insert(heads, tails, mp)
    }
}
