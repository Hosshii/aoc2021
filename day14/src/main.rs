use std::collections::{BTreeMap, HashMap, LinkedList};

fn main() {
    let ipt = include_str!("../input/input");
    let (polymer, def) = parse(ipt);

    let solved = (0..40).fold(polymer, |acc, _| solve(&acc, &def));

    let iter = ipt.lines().next().unwrap().chars();
    let chars = (iter.clone().next().unwrap(), iter.last().unwrap());
    let result = count(&solved, chars);
    println!("{}", result);
}

fn solve(
    polymer: &HashMap<(char, char), u64>,
    def: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut result = HashMap::new();
    for (k, v) in polymer.iter() {
        if let Some(d) = def.get(k) {
            *result.entry((k.0, *d)).or_insert(0) += *v;
            *result.entry((*d, k.1)).or_insert(0) += *v;
        } else {
            *result.entry(*k).or_insert(0) += *v;
        }
    }
    result
}

fn count(list: &HashMap<(char, char), u64>, (first, last): (char, char)) -> u64 {
    let mut tree = list
        .iter()
        .fold(BTreeMap::new(), |mut acc, ((c1, c2), &num)| {
            acc.entry(c1).and_modify(|x| *x += num).or_insert(num);
            acc.entry(c2).and_modify(|x| *x += num).or_insert(num);
            acc
        });
    *tree.entry(&first).or_insert(0) += 1;
    *tree.entry(&last).or_insert(0) += 1;
    println!("{:?}", tree);
    let max = tree.iter().max_by_key(|(_, v)| *v).unwrap();
    let min = tree.iter().min_by_key(|(_, v)| *v).unwrap();
    (max.1 - min.1) / 2
}

fn parse(ipt: &str) -> (HashMap<(char, char), u64>, HashMap<(char, char), char>) {
    let mut lines = ipt.lines();
    let mut template = lines.next().unwrap().chars().peekable();
    let mut def = HashMap::new();
    while let (Some(lhs), Some(&rhs)) = (template.next(), template.peek()) {
        def.entry((lhs, rhs)).and_modify(|x| *x += 1).or_insert(1);
    }

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
    (def, mp)
}
