use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

const NODE_COUNT: u32 = 200002;
const LOG_OF_N: usize = 18;

fn dfs<'a>(
    adjacency_list: &'a HashMap<u32, Vec<u32>>,
    depth_set: &'a mut HashMap<u32, u32>,
    root: u32,
    visited: &'a mut HashSet<u32>,
    depth: u32,
    parent_set: &'a mut HashMap<u32, u32>,
) {
    depth_set.insert(root, depth);
    visited.insert(root);
    for child in adjacency_list.get(&root).unwrap() {
        if !visited.contains(child) {
            parent_set.insert(*child, root);
            dfs(
                adjacency_list,
                depth_set,
                *child,
                visited,
                depth + 1,
                parent_set,
            );
        }
    }
}

fn sparse_table(
    adjacency_list: &HashMap<u32, Vec<u32>>,
) -> (Vec<Vec<u32>>, HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut depth_set: HashMap<u32, u32> = HashMap::new();
    let mut parent_set: HashMap<u32, u32> = HashMap::new();

    dfs(
        adjacency_list,
        &mut depth_set,
        1,
        &mut HashSet::new(),
        0,
        &mut parent_set,
    );
    let mut table = vec![vec![0; LOG_OF_N]; NODE_COUNT as usize];
    for i in 0..NODE_COUNT {
        table[i as usize][0] = *parent_set.get(&(i as u32)).unwrap_or(&0);
    }

    for j in 1..(LOG_OF_N as u32) {
        for i in 0..(NODE_COUNT as u32) {
            table[i as usize][j as usize] =
                table[table[i as usize][(j - 1) as usize] as usize][(j - 1) as usize];
        }
    }

    return (table, depth_set, parent_set);
}

fn lca(
    mut u: usize,
    mut v: usize,
    table: &Vec<Vec<u32>>,
    depth_set: &HashMap<u32, u32>,
    parent_set: &HashMap<u32, u32>,
) -> Option<u64> {
    let depth_u = depth_set.get(&(u as u32));
    let depth_v = depth_set.get(&(v as u32));

    if let (Some(depth_u), Some(depth_v)) = (depth_u, depth_v) {
        if depth_u > depth_v {
            std::mem::swap(&mut u, &mut v);
        }
    } else {
        return None;
    }

    // // move v up to the same depth as u
    for j in (0..LOG_OF_N).rev() {
        if depth_set[&(v as u32)] >= depth_set[&(u as u32)] + (1 << j) {
            v = table[v as usize][j] as usize;
        }
    }
    // // if u is an ancestor of v, return u
    if u == v {
        let depth_lca = depth_set[&(u as u32)];
        let depth = (((depth_u.unwrap() - depth_lca) as i32).abs()
            + ((depth_v.unwrap() - depth_lca) as i32).abs()) as u64;
        return Some(depth);
    }

    // // // move both nodes up to their lowest common ancestor
    for j in (0..LOG_OF_N).rev() {
        if table[u][j] != table[v][j] {
            u = table[u][j] as usize;
            v = table[v][j] as usize;
        }
    }
    // the lowest common ancestor is the parent of u or v
    let depth_lca = depth_set[&(parent_set[&(u as u32)] as u32)];
    let depth = (((depth_u.unwrap() - depth_lca) as i32).abs()
        + ((depth_v.unwrap() - depth_lca) as i32).abs()) as u64;
    return Some(depth);
}

fn print_output(adjacency_list: &HashMap<u32, Vec<u32>>, query_set: Vec<Vec<(u32, u32)>>) {
    let (table, depth_set, parent_set) = sparse_table(adjacency_list);

    for query_pairs in query_set {
        if query_pairs.len() == 0 {
            println!("0");
            continue;
        }
        let mut sum: i64 = 0;
        for (u, v) in query_pairs {
            if let Some(distance) = lca(u as usize, v as usize, &table, &depth_set, &parent_set) {
                sum += distance as i64;
            }
        }
        println!("{}", sum % 1000000007);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut query_set: Vec<Vec<(u32, u32)>> = Vec::new();

    // let counts = lines
    //     .next()
    //     .unwrap()
    //     .split_whitespace()
    //     .map(|i| i.parse::<u32>().unwrap())
    //     .collect::<Vec<u32>>();
    // let st = lines
    // .next().unwrap_or_default();
   
    println!("Sparse table == {:?}", lines.count());
    return;

    // let (nodes_count, query_count) = (counts[0], counts[1]);
    // for i in 0..nodes_count - 1 {
    //     let node_pair = lines
    //         .next()
    //         .unwrap()
    //         .split_whitespace()
    //         .map(|n| n.parse::<u32>().unwrap())
    //         .collect::<Vec<u32>>();

    //     adjacency_list
    //         .entry(node_pair[0])
    //         .or_insert(vec![node_pair[1]])
    //         .push(node_pair[1]);

    //     adjacency_list
    //         .entry(node_pair[1])
    //         .or_insert(vec![node_pair[1]])
    //         .push(node_pair[0]);
    // }

    // for _ in 0..query_count {
    //     lines.next();
    //     let mut queries_pair: Vec<(u32, u32)> = Vec::new();

    //     let queries = lines
    //         .next()
    //         .unwrap()
    //         .split_whitespace()
    //         .map(|q| q.parse::<u32>().unwrap())
    //         .collect::<Vec<u32>>();

    //     for (i, u) in queries.iter().enumerate() {
    //         for v in queries.iter().skip(i + 1) {
    //             queries_pair.push((*u, *v));
    //         }
    //     }
    //     query_set.push(queries_pair);
    // }

    // print_output(&adjacency_list, query_set);
}