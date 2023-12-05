use std::{fs::File, io::{self, BufRead}};
use itertools::Itertools;

struct CrazyMap {
    v: Vec<(u64, u64, u64)>
}

impl CrazyMap {
    fn new(v: Vec<(u64, u64, u64)>) -> Self {
        Self { v }
    }

    fn get(&self, n: &u64) -> u64 {
        self.v.iter()
            .find(|(_dest, source, len)| *n >= *source && *n < *source + *len)
            .map(|(dest, source, _len)| *n + *dest - *source)
            .or(Some(*n))
            .unwrap()
    }
}

struct CrazyList {
    l: Vec<CrazyMap>
}

impl CrazyList {
    fn new() -> Self {
        Self { l: Vec::new() }
    }

    fn add(&mut self, m: CrazyMap) {
        self.l.push(m);
    }

    fn get(&self, n: &u64) -> u64 {
        let mut res = *n;
        for i in 0..self.l.len() {
            res = self.l[i].get(&res);
        }
        res
    }
}

fn parse_input() -> (Vec<u64>, CrazyList) {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    
    // Seeds
    let seeds = lines.next().unwrap().unwrap();
    let seeds = seeds.split(":").nth(1).unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    lines.next();

    // Maps
    let mut list = CrazyList::new();
    let mut v = Vec::new();
    lines.for_each(|line| {
        let line = line.unwrap();
        if line.len() > 0 {
            if !line.contains(":") {
                let mut split = line.split_whitespace();
                v.push((
                    split.next().unwrap().parse::<u64>().unwrap(),
                    split.next().unwrap().parse::<u64>().unwrap(),
                    split.next().unwrap().parse::<u64>().unwrap()
                ));
            }
        } else {
            list.add(CrazyMap::new(v.clone()));
            v = Vec::new();
        }
    });

    (seeds, list)
}

fn main() {
    // First part
    let (seeds, list) = parse_input();
    let res = seeds.iter()
        .map(|s| list.get(s))
        .min().unwrap();

    println!("Result: {}", res);

    // Second part
    // Not proud of this brute-force solution, but didn't have time to properly optimize
    let res = seeds.iter().tuples::<(_, _)>()
        .flat_map(|(s, len)| *s .. *s + *len)
        .map(|s| list.get(&s))
        .min().unwrap();

    println!("Result: {}", res);
}
