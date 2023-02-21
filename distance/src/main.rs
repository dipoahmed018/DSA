use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::vec;

#[derive(Debug)]
struct Node {
    pub data: i32,
    pub childrens: Vec<Box<Node>>,
}

fn find_lca<'a, 'b>(
    root: &'a Node,
    nodes: (i32, i32),
    ancestor: &'b mut Vec<i32>,
) -> Option<&'a Node> {
    if root.data == nodes.0 || root.data == nodes.1 {
        if !ancestor.is_empty() {
            return Some(root);
        } else {
            ancestor.push(root.data);
        }
    }

    for child in &root.childrens {
        let found = find_lca(&child, nodes, ancestor);
        match found {
            Some(node) => {
                if let Some(ancestor_data) = ancestor.first() {
                    if *ancestor_data == node.data {
                        return Some(node);
                    } else {
                        return Some(root);
                    }
                }
            }
            _ => {
                if let Some(ancestor_data) = ancestor.first() {
                    if child.data == *ancestor_data {
                        ancestor.pop();
                        ancestor.push(root.data);
                    }
                }
            }
        }
    }

    return None;
}

fn insert(root: &mut Node, parent: i32, child: i32) {
    let mut q: VecDeque<&mut Node> = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        if n.data == parent {
            n.childrens.push(Box::new(Node {
                data: child,
                childrens: vec![],
            }));
        };

        for children in n.childrens.iter_mut() {
            q.push_back(children);
        }
    }
}

fn main() {
    let file = File::open("inputs.text").unwrap();

    let mut input_string = String::new();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut input_string);

    let mut lines = input_string.lines();
    let counts = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let i_count: usize = counts[0];
    let q_count: usize = counts[1];

    let parent_pair = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut root = Node {
        data: parent_pair[0],
        childrens: vec![],
    };

    root.childrens.push(Box::new(Node {
        data: parent_pair[1],
        childrens: vec![],
    }));

    for i in 0..(i_count - 1) {
        let input_pair: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if input_pair[0] == input_pair[1] {
            continue;
        }
        insert(&mut root, input_pair[0], input_pair[1]);
    }

    let mut queryset: Vec<HashMap<i32, bool>> = vec![];
    for n in 0..q_count {
        lines.next();
        let mut queries: HashMap<i32, bool> = HashMap::new();
        let inputs = lines.next().unwrap().split_whitespace();

        for input in inputs {
            let input = input.parse::<i32>().unwrap();
            queries.insert(input, true);
        }
        queryset.push(queries);
    }

    let mut ancestors: Vec<i32> = vec![];
    let nodes = (1, 7);
    let lca = find_lca(&root, nodes, &mut ancestors);

    print!("{:?}", lca);
}
