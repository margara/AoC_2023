use itertools::Itertools;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    usize,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    v1: usize,
    v2: usize,
}

impl Edge {
    fn new(v1: usize, v2: usize) -> Self {
        let (v1, v2) = if v1 < v2 { (v1, v2) } else { (v2, v1) };
        Self { v1, v2 }
    }
}

fn parse_input() -> HashMap<usize, Vec<usize>> {
    let filename = "input/input.txt";
    let f = File::open(filename).unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut dict = HashMap::new();
    lines.for_each(|line| {
        let line = line.unwrap();
        let line = line.replace(":", " ");
        let line = line.split_whitespace();
        line.for_each(|v| {
            let dict_len = dict.len();
            dict.entry(v.to_owned()).or_insert(dict_len);
        })
    });

    let f = File::open(filename).unwrap();
    let mut res = HashMap::new();
    let lines = io::BufReader::new(f).lines();
    lines.for_each(|line| {
        let line = line.unwrap();
        let mut line = line.split(":");
        let v1 = *dict.get(line.next().unwrap()).unwrap();
        let others = line.next().unwrap();
        let others = others.split_whitespace();
        others.for_each(|v2| {
            let v2 = *dict.get(v2).unwrap();
            res.entry(v1).or_insert_with(|| Vec::new()).push(v2);
            res.entry(v2).or_insert_with(|| Vec::new()).push(v1);
        })
    });

    res
}

fn connected_without(graph: &HashMap<usize, Vec<usize>>, edges: &[&Edge]) -> usize {
    let mut graph = graph.clone();
    // TODO
    edges.iter().for_each(|e| {
        graph.get_mut(&e.v1).unwrap().retain(|v| *v != e.v2);
        graph.get_mut(&e.v2).unwrap().retain(|v| *v != e.v1);
    });

    let mut all: HashSet<usize> = HashSet::new();
    all.insert(0);
    let mut new: HashSet<usize> = all.clone();
    loop {
        new = new
            .iter()
            .flat_map(|v| graph.get(v).unwrap())
            .filter(|v| !all.contains(v))
            .cloned()
            .collect::<HashSet<_>>();
        if new.is_empty() {
            break;
        } else {
            all.extend(new.iter())
        }
    }

    all.len()
}

fn bfs(graph: &HashMap<usize, Vec<usize>>, v1: usize, v2: usize) -> Vec<usize> {
    let mut paths = vec![vec![v1]];
    let mut res = Vec::new();

    loop {
        if paths.is_empty() || !res.is_empty() || paths.len() > 2000 {
            break;
        }

        // TODO
        let path = paths.remove(0);
        for v in &graph[path.last().unwrap()] {
            if !path.contains(v) {
                let mut new_path = path.clone();
                new_path.push(*v);
                if *v == v2 {
                    res = new_path;
                } else {
                    paths.push(new_path);
                }
            }
        }
    }

    res
}

fn test_random_vertices(
    graph: &HashMap<usize, Vec<usize>>,
    already_used: &mut HashSet<(usize, usize)>,
    counts: &mut HashMap<Edge, usize>,
) {
    let (mut v1, mut v2);
    loop {
        v1 = rand::thread_rng().gen_range(0..graph.len());
        v2 = rand::thread_rng().gen_range(0..graph.len());
        if !already_used.contains(&(v1, v2)) {
            already_used.insert((v1, v2));
            break;
        }
    }

    let path = bfs(graph, v1, v2);
    path.into_iter()
        .tuple_windows()
        .map(|(v1, v2)| Edge::new(v1, v2))
        .for_each(|e| *counts.entry(e).or_default() += 1);
}

fn main() {
    let graph = parse_input();

    // Part one
    let mut edges = HashMap::new();
    let mut already_used = HashSet::new();

    let mut i = 0;
    loop {
        println!("Loop {}", i);
        i += 1;
        for _ in 0..100 {
            test_random_vertices(&graph, &mut already_used, &mut edges);
        }
        let edges = edges
            .iter()
            .sorted_by_key(|(_e, c)| *c)
            .rev()
            .take(3)
            .map(|(e, _c)| e)
            .collect_vec();

        let connected = connected_without(&graph, &edges);
        if connected != graph.len() {
            println!("Graph len: {}", graph.len());
            let others = graph.len() - connected;
            println!(
                "Set1: {}, set2: {}, product: {}",
                connected,
                others,
                connected * others
            );
            break;
        }
    }
}
