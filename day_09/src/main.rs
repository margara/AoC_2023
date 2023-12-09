use std::{fs::File, io::{self, BufRead}};
use itertools::Itertools;

fn parse_input() -> Vec<Vec<i64>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect_vec()
    }).collect_vec()
}

fn compute_differences(v: Vec<i64>) -> Vec<Vec<i64>> {
    let mut res = Vec::new();
    let mut new = &v;
    res.push(v.clone());
    loop {
        let next = new.iter().tuple_windows().map(|(a, b)| *b - *a).collect_vec();
        res.push(next);
        new = res.last().unwrap();
        if new.iter().all(|x| *x == 0) {
            break;
        }
    }
    res
}

fn compute_last_value(v: Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for i in (0..v.len()).rev() {
        let inner = v.get(i).unwrap();
        let last_val = inner.last().unwrap();
        res += *last_val;
    }
    res
}

fn compute_first_value(v: Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for i in (0..v.len()).rev() {
        let inner = v.get(i).unwrap();
        let first_val = inner.first().unwrap();
        res = *first_val - res;
    }
    res
}

fn main() {
    // First part
    let input = parse_input();
    let res = input.into_iter()
        .map(|v| compute_differences(v))
        .map(|v| compute_last_value(v))
        .sum::<i64>();

    println!("Sum: {}", res);

    // Second part
    let input = parse_input();
    let res = input.into_iter()
        .map(|v| compute_differences(v))
        .map(|v| compute_first_value(v))
        .sum::<i64>();

    print!("Sum: {}", res);   
}