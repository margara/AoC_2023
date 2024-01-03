use bit_set::BitSet;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq)]
enum AllowedDirs {
    Up,
    Down,
    Left,
    Right,
    Any,
}

#[derive(Debug)]
struct Map {
    cells: Vec<Cell>,
}

impl Map {
    fn new(map: Vec<Vec<Option<AllowedDirs>>>) -> Self {
        let rows = map.len();
        let cols = map.first().unwrap().len();

        let mut coord_to_id = HashMap::new();
        let mut next_id = 0;
        for r in 0..rows {
            for c in 0..cols {
                if map[r][c].is_some() {
                    coord_to_id.insert((r, c), next_id);
                    next_id += 1;
                }
            }
        }

        let cells = coord_to_id
            .iter()
            .map(|((r, c), id)| {
                let mut adjacents = Vec::new();
                match map[*r][*c].as_ref().unwrap() {
                    AllowedDirs::Up => {
                        if *r > 0 && map[*r - 1][*c].is_some() {
                            let id = *coord_to_id.get(&(*r - 1, *c)).unwrap();
                            adjacents.push(id);
                        }
                    }
                    AllowedDirs::Down => {
                        if *r < rows - 1 && map[*r + 1][*c].is_some() {
                            let id = *coord_to_id.get(&(*r + 1, *c)).unwrap();
                            adjacents.push(id);
                        }
                    }
                    AllowedDirs::Left => {
                        if map[*r][*c - 1].is_some() {
                            let id = *coord_to_id.get(&(*r, *c - 1)).unwrap();
                            adjacents.push(id);
                        }
                    }
                    AllowedDirs::Right => {
                        if map[*r][*c + 1].is_some() {
                            let id = *coord_to_id.get(&(*r, *c + 1)).unwrap();
                            adjacents.push(id);
                        }
                    }
                    AllowedDirs::Any => {
                        if *r > 0 && map[*r - 1][*c].is_some() {
                            let id = *coord_to_id.get(&(*r - 1, *c)).unwrap();
                            adjacents.push(id);
                        }
                        if *r < rows - 1 && map[*r + 1][*c].is_some() {
                            let id = *coord_to_id.get(&(*r + 1, *c)).unwrap();
                            adjacents.push(id);
                        }
                        if map[*r][*c - 1].is_some() {
                            let id = *coord_to_id.get(&(*r, *c - 1)).unwrap();
                            adjacents.push(id);
                        }
                        if map[*r][*c + 1].is_some() {
                            let id = *coord_to_id.get(&(*r, *c + 1)).unwrap();
                            adjacents.push(id);
                        }
                    }
                }
                (id, Cell::new(adjacents))
            })
            .sorted_by_key(|(id, _)| **id)
            .map(|(_, cell)| cell)
            .collect::<Vec<_>>();

        Self { cells }
    }
}

#[derive(Debug)]
struct Cell {
    adjacents: Vec<usize>,
}

impl Cell {
    fn new(adjacents: Vec<usize>) -> Self {
        Self { adjacents }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Path {
    visited: BitSet,
    last: usize,
}

impl Path {
    fn new() -> Self {
        let mut visited = BitSet::new();
        visited.insert(0);
        let last = 0;

        Self { visited, last }
    }

    fn extend(&self, cell: usize) -> Path {
        let mut visited = self.visited.clone();
        visited.insert(cell);
        let last = cell;

        Self { visited, last }
    }
}

fn find_longest_path(map: &Map) -> usize {
    let mut paths = vec![Path::new()];
    let mut longest = 0;

    loop {
        if paths.is_empty() {
            break;
        }

        let path = paths.pop().unwrap();
        map.cells[path.last]
            .adjacents
            .iter()
            .filter(|cell| !path.visited.contains(**cell))
            .map(|&cell| {
                let new_path = path.extend(cell);
                if cell == map.cells.len() - 1 && new_path.visited.len() > longest {
                    longest = new_path.visited.len();
                    println!("New longest {}", longest - 1);
                }
                new_path
            })
            .for_each(|new_path| {
                paths.push(new_path);
            });
    }

    longest - 1
}

fn parse_input() -> Vec<Vec<Option<AllowedDirs>>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    '#' => None,
                    '.' => Some(AllowedDirs::Any),
                    '^' => Some(AllowedDirs::Up),
                    'v' => Some(AllowedDirs::Down),
                    '<' => Some(AllowedDirs::Left),
                    '>' => Some(AllowedDirs::Right),
                    _ => panic!("Parse error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_input2() -> Vec<Vec<Option<AllowedDirs>>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    '#' => None,
                    '.' | '^' | 'v' | '<' | '>' => Some(AllowedDirs::Any),
                    _ => panic!("Parse error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    // First part
    let map = Map::new(parse_input());
    let longest = find_longest_path(&map);
    println!("Longest path: {}", longest);

    // Second part
    let map = Map::new(parse_input2());
    let longest = find_longest_path(&map);
    println!("Longest path: {}", longest);
}
