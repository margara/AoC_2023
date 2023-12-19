use std::{
    fmt::{Display, Write},
    fs::File,
    io::{self, BufRead}, collections::{HashSet, HashMap},
};

#[derive(Hash, PartialEq, Eq, Clone)]
enum Block {
    Round,
    Cube,
    Empty,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Platform {
    map: Vec<Vec<Block>>,
    rows: usize,
    cols: usize,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.iter().for_each(|row| {
            row.iter().for_each(|b| match b {
                Block::Cube => {
                    f.write_char('#').unwrap();
                }
                Block::Round => {
                    f.write_char('O').unwrap();
                }
                Block::Empty => {
                    f.write_char('.').unwrap();
                }
            });
            f.write_str("\n").unwrap();
        });
        Ok(())
    }
}

impl Platform {
    fn new(map: Vec<Vec<Block>>) -> Self {
        let cols = map.len();
        let rows = map.first().expect("Empty map").len();
        Self { map, rows, cols }
    }

    fn tilt_north(&mut self) {
        fn tilt_north_one_step(platform: &mut Platform) -> bool {
            let mut changed = false;
            for r in 1..platform.rows {
                for c in 0..platform.cols {
                    if platform.map[r][c] == Block::Round && platform.map[r-1][c] == Block::Empty
                    {
                        platform.map[r][c] = Block::Empty;
                        platform.map[r-1][c] = Block::Round;
                        changed = true;
                    }
                }
            }
            changed
        }

        loop {
            let changed = tilt_north_one_step(self);
            if !changed {
                break;
            }
        }
    }

    fn tilt_south(&mut self) {
        fn tilt_south_one_step(platform: &mut Platform) -> bool {
            let mut changed = false;
            for r in (0..platform.rows-1).rev() {
                for c in 0..platform.cols {
                    if platform.map[r][c] == Block::Round && platform.map[r+1][c] == Block::Empty
                    {
                        platform.map[r][c] = Block::Empty;
                        platform.map[r+1][c] = Block::Round;
                        changed = true;
                    }
                }
            }
            changed
        }

        loop {
            let changed = tilt_south_one_step(self);
            if !changed {
                break;
            }
        }
    }

    fn tilt_west(&mut self) {
        fn tilt_west_one_step(platform: &mut Platform) -> bool {
            let mut changed = false;
            for r in 0..platform.rows {
                for c in 1..platform.cols {
                    if platform.map[r][c] == Block::Round && platform.map[r][c-1] == Block::Empty
                    {
                        platform.map[r][c] = Block::Empty;
                        platform.map[r][c-1] = Block::Round;
                        changed = true;
                    }
                }
            }
            changed
        }

        loop {
            let changed = tilt_west_one_step(self);
            if !changed {
                break;
            }
        }
    }

    fn tilt_east(&mut self) {
        fn tilt_east_one_step(platform: &mut Platform) -> bool {
            let mut changed = false;
            for r in 0..platform.rows {
                for c in (0..platform.cols-1).rev() {
                    if platform.map[r][c] == Block::Round && platform.map[r][c+1] == Block::Empty
                    {
                        platform.map[r][c] = Block::Empty;
                        platform.map[r][c+1] = Block::Round;
                        changed = true;
                    }
                }
            }
            changed
        }

        loop {
            let changed = tilt_east_one_step(self);
            if !changed {
                break;
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn compute_total_load(&self) -> usize {
        (0..self.rows)
            .map(|r| {
                let r_weight = self.rows - r;
                let num_rounds = self.map[r].iter().filter(|b| **b == Block::Round).count();
                num_rounds * r_weight
            })
            .sum::<usize>()
    }
}

fn parse_input() -> Platform {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let map = lines
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| match c {
                    '.' => Block::Empty,
                    '#' => Block::Cube,
                    'O' => Block::Round,
                    _ => panic!("Unknown block"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Platform::new(map)
}

fn main() {
    let mut platform = parse_input();

    // First part
    println!("Platform before:\n{}\n", platform);
    platform.tilt_north();
    println!("Platform after:\n{}\n", platform);
    let total_load = platform.compute_total_load();
    println!("Total load: {}", total_load);

    // Second part
    let mut platform = parse_input();
    let mut previous = HashMap::new();
    (0..200).for_each(|i| {
        platform.cycle();
        let c = platform.clone();
        if previous.contains_key(&c) {
            println!("State at {} equal to state at {}", i, previous.get(&c).unwrap());
        } else {
            previous.insert(c, i);
        }
    });

    let mut state = 130;
    let mut equivalent_state = 108;
    (131..1_000_000_001).for_each(|_| {
        state += 1;
        equivalent_state += 1;
        if equivalent_state == 130 {
            equivalent_state = 108;
        }
    });
    println!("State {} equal to state {}", state, equivalent_state);

    let mut platform = parse_input();
    (0..equivalent_state).for_each(|_| {
        platform.cycle();
    });

    let total_load = platform.compute_total_load();
    println!("Total load at state {}: {}", equivalent_state, total_load);
}
