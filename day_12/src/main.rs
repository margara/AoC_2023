use rayon::{iter::{ParallelIterator}, prelude::*};
use std::{
    fs::File,
    io::{self, BufRead},
    usize, cell::RefCell, collections::HashMap,
};

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

fn is_compatible(generated_pattern: &[Spring], left: &[Spring]) -> bool {
    if generated_pattern.len() != left.len() {
        return false;
    }

    for i in 0..generated_pattern.len() {
        if left[i] != Spring::U && left[i] != generated_pattern[i] {
            return false;
        }
    }
    true
}

fn generate_pattern(generated: &[usize], right: &[usize]) -> Vec<Spring> {
    let mut res = Vec::new();
    for i in 0..generated.len() {
        for _o in 0..generated[i] {
            res.push(Spring::O);
        }
        if i != 0 && i != generated.len() - 1 {
            res.push(Spring::O);
        }
        if i != generated.len() - 1 {
            for _d in 0..right[i] {
                res.push(Spring::D);
            }
        }
    }

    res
}

fn generate_sets_memo(remaining_ops: usize, remaining_groups: usize, memo: &RefCell<HashMap<(usize, usize), Vec<Vec<usize>>>>) -> Vec<Vec<usize>> {
    if remaining_groups == 1 {
        if !memo.borrow().contains_key(&(remaining_ops, remaining_groups)) {
            let next = (0..remaining_ops+1).map(|ops| vec![ops]).collect::<Vec<_>>();
            memo.borrow_mut().insert((remaining_ops, remaining_groups), next);
        }
        let map = memo.borrow();
        map.get(&(remaining_ops, remaining_groups)).unwrap().clone()
    } else {
        (0..remaining_ops+1).flat_map(|ops| {
            if !memo.borrow().contains_key(&(remaining_ops, remaining_groups)) {
                let next = generate_sets_memo(remaining_ops - ops, remaining_groups - 1, memo);
                memo.borrow_mut().insert((remaining_ops, remaining_groups), next);
            }
            let map = memo.borrow();
            let next = map.get(&(remaining_ops, remaining_groups)).unwrap();
            next.iter().map(|n| {
                let mut n = n.clone();
                n.push(ops);
                n
            }).collect::<Vec<_>>()
        }).collect()
    }
}

fn generate_sets(
    current_group: usize,
    total_groups: usize,
    remaining_ops: usize,
    current: &Vec<usize>,
) -> Vec<Vec<usize>> {
    (0..remaining_ops + 1)
        .flat_map(|ops| {
            let mut current = current.clone();
            current.push(ops);
            let remaining_ops = remaining_ops - ops;
            if current_group == total_groups - 1 {
                vec![current]
            } else {
                generate_sets(current_group + 1, total_groups, remaining_ops, &current)
            }
        })
        .collect::<Vec<_>>()
}

fn process_spring(left: &[Spring], right: &[usize]) -> usize {
    let num_operational_groups = right.len() + 1;
    let left_len = left.len();
    let num_damaged = right.iter().sum::<usize>();
    let ops_to_add = left_len - num_damaged - (right.len() - 1);

    let memo = RefCell::new(HashMap::new());

    let sets = generate_sets_memo(ops_to_add, num_operational_groups, &memo);
    // let sets = generate_sets(0, num_operational_groups, ops_to_add, &Vec::new());
    let res = sets
        .into_iter()
        .map(|s| generate_pattern(&s, right))
        .filter(|p| is_compatible(p, left))
        .count();

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
        .iter()
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
