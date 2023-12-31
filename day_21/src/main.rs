use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    G,
    R,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(map: Vec<Vec<Tile>>) -> Self {
        let rows = map.len();
        let cols = map.first().unwrap().len();

        Self { map, rows, cols }
    }
}

fn compute_reachability(points: &HashSet<Point>, map: &Map) -> HashSet<Point> {
    points
        .iter()
        .flat_map(|p| {
            let mut reachable = HashSet::new();
            let r = p.row as usize;
            let c = p.col as usize;
            if r > 0 && map.map[r - 1][c] == Tile::G {
                reachable.insert(Point::new(p.row - 1, p.col));
            }
            if r < map.rows - 1 && map.map[r + 1][c] == Tile::G {
                reachable.insert(Point::new(p.row + 1, p.col));
            }
            if c > 0 && map.map[r][c - 1] == Tile::G {
                reachable.insert(Point::new(p.row, p.col - 1));
            }
            if c < map.cols - 1 && map.map[r][c + 1] == Tile::G {
                reachable.insert(Point::new(p.row, p.col + 1));
            }
            reachable
        })
        .collect::<HashSet<_>>()
}

fn compute_reachability_infinite(points: &HashSet<Point>, map: &Map) -> HashSet<Point> {
    points
        .iter()
        .flat_map(|p| {
            let mut reachable = HashSet::new();
            let r = p.row;
            let c = p.col;
            if map.map[inf_mod(r - 1, map.rows)][inf_mod(c, map.cols)] == Tile::G {
                reachable.insert(Point::new(r - 1, c));
            }
            if map.map[inf_mod(r + 1, map.rows)][inf_mod(c, map.cols)] == Tile::G {
                reachable.insert(Point::new(r + 1, c));
            }
            if map.map[inf_mod(r, map.rows)][inf_mod(c - 1, map.cols)] == Tile::G {
                reachable.insert(Point::new(r, c - 1));
            }
            if map.map[inf_mod(r, map.rows)][inf_mod(c + 1, map.cols)] == Tile::G {
                reachable.insert(Point::new(r, c + 1));
            }
            reachable
        })
        .collect::<HashSet<_>>()
}

fn inf_mod(n: i64, size: usize) -> usize {
    let size = size as i64;
    let res = ((n % size) + size) % size;
    res as usize
}

fn parse_input() -> (Map, Point) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut start_point = (0, 0);
    let map = lines
        .enumerate()
        .map(|(row, line)| {
            let line = line.unwrap();
            line.char_indices()
                .map(|(col, c)| match c {
                    '.' => Tile::G,
                    'S' => {
                        start_point = (row, col);
                        Tile::G
                    }
                    '#' => Tile::R,
                    _ => panic!("Parse error"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (
        Map::new(map),
        Point::new(start_point.0 as i64, start_point.1 as i64),
    )
}

fn main() {
    let (map, start_point) = parse_input();

    // Part one
    let mut reachable = HashSet::new();
    reachable.insert(start_point.clone());
    (0..64).for_each(|_| {
        reachable = compute_reachability(&reachable, &map);
    });
    println!("Reachable in 64 steps: {}", reachable.len());

    // Part two (simpler solution after reading the solutions sub reddit :)
    let mut reachable = HashSet::new();
    reachable.insert(start_point.clone());
    let x = (0..3).map(|i| 131 * i + 65).collect::<Vec<_>>();
    let xmax = 131 * 2 + 65;
    let y = (1..xmax + 1)
        .map(|i| {
            reachable = compute_reachability_infinite(&reachable, &map);
            println!("Step: {}, val: {}", i, reachable.len());
            (i, reachable.len())
        })
        .collect::<HashMap<_, _>>();

    let x0 = x[0] as f64;
    let x1 = x[1] as f64;
    let x2 = x[2] as f64;

    let y0 = *y.get(&x[0]).unwrap() as f64;
    let y1 = *y.get(&x[1]).unwrap() as f64;
    let y2 = *y.get(&x[2]).unwrap() as f64;

    let y01 = (y1 - y0) / (x1 - x0);
    let y12 = (y2 - y1) / (x2 - x1);
    let y012 = (y12 - y01) / (x2 - x0);

    let steps = 26501365_f64;
    let res = y0 + y01 * (steps - x0) + y012 * (steps - x0) * (steps - x1);
    println!("Reachable in 26501365 steps: {}", res as u64);
}
