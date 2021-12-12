use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let ipt = include_str!("../input/input");
    let parsed = parse(ipt);

    let paths = solve(parsed, "start");

    println!("{}", paths.len());
    // for path in paths {
    //     for node in path {
    //         println!("{}", node.borrow().name);
    //     }
    //     println!();
    // }
    // println!("{:?}", parsed);
}

// fn solve(map: HashMap<&str, NodeRef>) -> Vec<Path> {
//     let start = "start";
//     let start_node = map.get(start).expect("start node not found");
//     for neighbor in start_node.borrow().neighbors {}
// }

fn solve(mut map: HashMap<&str, (NodeRef, bool)>, name: &str) -> Vec<Path> {
    let (cur, visited) = map.get_mut(name).expect("node not found");
    if cur.borrow().type_ == NodeType::Small {
        *visited = true;
    }

    let (cur, _) = map.get(name).expect("node not found");

    let mut result = vec![];
    if name == "end" {
        result.push(vec![Rc::clone(cur)]);
        return result;
    }

    for neighbor in &cur.borrow().neighbors {
        let visited = map
            .get(&neighbor.borrow().name.as_str())
            .expect("neighbor not found")
            .1;
        if !visited {
            // println!("hh {}", neighbor.borrow().name);
            let name = neighbor.borrow().name.clone();
            let mut paths = solve(map.clone(), &name);
            paths.iter_mut().for_each(|path| path.push(Rc::clone(cur)));
            result.append(&mut paths);
        }
    }
    result
}

fn parse(ipt: &str) -> HashMap<&str, (NodeRef, bool)> {
    let mut map = HashMap::new();
    for line in ipt.lines() {
        let mut splitted = line.split('-');
        let (lhs, rhs) = (splitted.next().unwrap(), splitted.next().unwrap());
        if map.get(lhs).is_none() {
            insert_node(&mut map, lhs);
        }
        if map.get(rhs).is_none() {
            insert_node(&mut map, rhs);
        }

        let lhs = &map.get(lhs).unwrap().0;
        let rhs = &map.get(rhs).unwrap().0;
        lhs.borrow_mut().neighbors.push(Rc::clone(rhs));
        rhs.borrow_mut().neighbors.push(Rc::clone(lhs));
    }
    map
}

fn insert_node<'a, 'b>(map: &'a mut HashMap<&'b str, (NodeRef, bool)>, name: &'b str)
where
    'b: 'a,
{
    let ty = if name == name.to_uppercase() {
        NodeType::Big
    } else {
        NodeType::Small
    };
    map.insert(name, (Node::new_rc(name, ty), false));
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    type_: NodeType,
    neighbors: Vec<NodeRef>,
}

type NodeRef = Rc<RefCell<Node>>;

type Path = Vec<NodeRef>;

impl Node {
    fn new(name: &str, type_: NodeType) -> Node {
        Node {
            name: name.to_string(),
            type_,
            neighbors: Vec::new(),
        }
    }

    fn new_rc(name: &str, type_: NodeType) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(name, type_)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Big,
    Small,
}
