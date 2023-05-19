use core::f32;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::vec;

fn dfs(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    euler_tree: &mut Vec<u32>,
    depth_set: &mut HashMap<u32, u32>,
    first_index: &mut HashMap<u32, usize>,
    first_parent: &mut HashMap<u32, usize>,
    visited: &mut HashSet<u32>,
    root: u32,
    depth: u32,
) {
    visited.insert(root);
    depth_set.insert(root, depth);
    euler_tree.push(root);
    first_index.insert(root, euler_tree.len() - 1);

    for child in adjacency_list.get(&root).unwrap() {
        if !visited.contains(child) {
            first_parent.insert(*child, root as usize);
            dfs(
                adjacency_list,
                euler_tree,
                depth_set,
                first_index,
                first_parent,
                visited,
                *child,
                depth + 1,
            );
            euler_tree.push(root);
        }
    }
}

fn build_sparse_table(
    euler_tree: &mut Vec<u32>,
    depth_set: &mut HashMap<u32, u32>,
) -> Vec<Vec<i32>> {
    let log_n = (f32::log2((euler_tree.len()) as f32).floor() as usize) + 1;
    let mut sparse_table: Vec<Vec<i32>> = vec![vec![-1; log_n]; euler_tree.len()];

    //initialize first parents
    for i in 0..euler_tree.len() {
        sparse_table[i][0] = euler_tree[i] as i32;
    }

    for j in 1..log_n {
        for i in 0..euler_tree.len() {
            if i + (1 << j) - 1 >= euler_tree.len() {
                continue;
            }
            let chunk_one = sparse_table[i][j - 1];
            let chunm_two = sparse_table[i + (1 << (j - 1))][j - 1];
            let depth_one = depth_set.get(&(chunk_one as u32));
            let depth_two = depth_set.get(&(chunm_two as u32));

            if let (Some(depth_one), Some(depth_two)) = (depth_one, depth_two) {
                sparse_table[i][j] = if depth_one < depth_two {
                    chunk_one as i32
                } else {
                    chunm_two as i32
                };
            }
        }
    }
    return sparse_table;
}

fn query(
    l: usize,
    r: usize,
    saprse_table: &Vec<Vec<i32>>,
    first_index: &HashMap<u32, usize>,
    depth_set: &HashMap<u32, u32>,
) -> Option<u32> {
    let mut l_idx = first_index.get(&(l as u32));
    let mut r_idx = first_index.get(&(r as u32));

    if l_idx == r_idx {
        return Some(l as u32);
    }

    if l_idx > r_idx {
        std::mem::swap(&mut l_idx, &mut r_idx);
    }
    if let (Some(&l_idx), Some(&r_idx)) = (l_idx, r_idx) {
        let j = (f32::log2((r_idx - l_idx) as f32).floor()) as usize;

        let first_chunk_depth = *depth_set
            .get(&(saprse_table[l_idx][j] as u32))
            .unwrap_or(&std::u32::MAX);
        let second_chunk_depth = *depth_set
            .get(&(saprse_table[r_idx - (1 << j) + 1][j] as u32))
            .unwrap_or(&std::u32::MAX);

        if first_chunk_depth < second_chunk_depth {
            return Some(saprse_table[l_idx][j] as u32);
        } else {
            return Some(saprse_table[r_idx - (1 << j) + 1][j] as u32);
        }
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let counts = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let (nodes_count, query_count) = (counts[0], counts[1]);

    for i in 1..nodes_count {
        let node_pair = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        adjacency_list
            .entry(node_pair[0])
            .or_insert(vec![node_pair[0]])
            .push(node_pair[1]);

        adjacency_list
            .entry(node_pair[1])
            .or_insert(vec![node_pair[1]])
            .push(node_pair[0]);
    }

    let mut euler_tree: Vec<u32> = Vec::new();
    let mut first_index: HashMap<u32, usize> = HashMap::new();
    let mut first_parent: HashMap<u32, usize> = HashMap::new();
    let mut depth_set: HashMap<u32, u32> = HashMap::new();
    let mut visited: HashSet<u32> = HashSet::new();
    dfs(
        &adjacency_list,
        &mut euler_tree,
        &mut depth_set,
        &mut first_index,
        &mut first_parent,
        &mut visited,
        1,
        0,
    );

    let sparse_table = build_sparse_table(&mut euler_tree, &mut depth_set);
    for _ in 0..query_count {
        lines.next();

        let queries = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|q| q.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut sum: i128 = 0;
        for i in 0..queries.len() {
            for j in i + 1..queries.len() {
                if let Some(lca) = query(
                    queries[i] as usize,
                    queries[j] as usize,
                    &sparse_table,
                    &first_index,
                    &depth_set,
                ) {
                    let lca_distance = depth_set.get(&(lca)).unwrap();
                    let &u_depth = depth_set.get(&(queries[i])).unwrap();
                    let &v_depth = depth_set.get(&(queries[j])).unwrap();

                    let distance = (u_depth + v_depth - lca_distance * 2) as i128;
                    sum += (queries[i] as i128 * queries[j] as i128 * distance) as i128;
                }
            }
        }
        println!("{sum}")
    }
}
