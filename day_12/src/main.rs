use std::{
    fs::File,
    io::{self, BufRead},
};
use rayon::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Spring {
    O,
    D,
    U,
}

fn parse_input() -> Vec<(Vec<Spring>, Vec<usize>)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let left = line.next().unwrap().to_string();
            let left = left
                .chars()
                .map(|c| match c {
                    '.' => Spring::O,
                    '#' => Spring::D,
                    '?' => Spring::U,
                    _ => panic!("Unknown spring type"),
                })
                .collect::<Vec<_>>();

            let right = line
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (left, right)
        })
        .collect::<Vec<_>>()
}

fn spring_ok(left: &[Spring], right: &[usize], missing_damaged: usize) -> bool {
    if missing_damaged == 0 {
        let mut left = left.to_owned();
        left.iter_mut()
            .filter(|s| **s == Spring::U)
            .for_each(|s| *s = Spring::O);
        let left = left.split(|el| *el == Spring::O);
        let left = left.map(|x| x.len()).filter(|x| *x > 0).collect::<Vec<_>>();
        left == right
    } else {
        let mut it = left.split(|el| *el == Spring::U);
        let prefix = it.next().unwrap();
        let prefix = prefix.split(|el| *el == Spring::O);
        let prefix = prefix
            .map(|x| x.len())
            .filter(|x| *x > 0)
            .collect::<Vec<_>>();

        prefix.len() <= right.len() &&
        (0..prefix.len()-1).all(|i| {
            prefix.get(i) == right.get(i)
        }) &&
        prefix.get(prefix.len()-1) <= right.get(prefix.len()-1)
    }
}

// There are still missing_damaged damaged springs
// I try to put the next damaged (and call recursively)
fn process_spring_rec(left: &[Spring], right: &[usize], missing_damaged: usize) -> usize {
    let mut res = 0;
    let num_unknown = left.iter().filter(|c| **c == Spring::U).count();

    for i in 0..num_unknown {
        let mut left = left.to_owned();
        left.iter_mut()
            .filter(|s| **s == Spring::U)
            .take(i + 1)
            .enumerate()
            .for_each(|(c, s)| {
                *s = if c == i { Spring::D } else { Spring::O };
            });

        let missing_damaged = missing_damaged - 1;
        if spring_ok(&left, &right, missing_damaged) {
            if missing_damaged == 0 {
                res += 1;
            } else {
                res += process_spring_rec(&left, &right, missing_damaged);
            }
        }
    }

    res
}

fn process_spring(left: &[Spring], right: &[usize]) -> usize {
    let current_damaged = left.iter().filter(|c| **c == Spring::D).count();
    let expected_damaged = right.iter().sum::<usize>();
    let missing_damaged = expected_damaged - current_damaged;

    let res = if missing_damaged == 0 { 
        1
    } else {
        process_spring_rec(left, right, missing_damaged)
    };

    println!(">> {}", res);
    res
}

fn main() {
    let input = parse_input();

    // First part
    let sum = input
        .iter()
        .map(|(left, right)| process_spring(left, right))
        .sum::<usize>();

    println!("Sum: {}", sum);

    // Second part
    let sum = input
        .par_iter()
        .map(|(left, right)| {
            let mut left = left.clone();
            left.push(Spring::U);
            left = left.repeat(5);
            left.pop();
            (left, right.repeat(5))
        })
        .map(|(left, right)| process_spring(&left, &right))
        .sum::<usize>();

    println!("Sum: {}", sum);
}
