use rayon::prelude::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    U,
    D,
    R,
    L,
}

enum Block {
    Empty,
    Horizontal,
    Vertical,
    RightUp,
    RightDown,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Beam {
    dir: Dir,
    row: usize,
    col: usize,
}

impl Beam {
    fn new(dir: Dir, row: usize, col: usize) -> Self {
        Self { dir, row, col }
    }
}

struct World {
    map: Vec<Vec<Block>>,
    rows: usize,
    cols: usize,
}

impl World {
    fn new(map: Vec<Vec<Block>>) -> Self {
        let rows = map.len();
        let cols = map.first().unwrap().len();
        Self { map, rows, cols }
    }

    fn mv_bean(&self, beam: &Beam) -> Vec<Beam> {
        let mut res = Vec::new();

        fn move_up(_world: &World, beam: &Beam, vec: &mut Vec<Beam>) {
            if beam.row > 0 {
                vec.push(Beam::new(Dir::U, beam.row - 1, beam.col));
            }
        }

        fn move_down(world: &World, beam: &Beam, vec: &mut Vec<Beam>) {
            if beam.row < world.rows - 1 {
                vec.push(Beam::new(Dir::D, beam.row + 1, beam.col));
            }
        }

        fn move_left(_world: &World, beam: &Beam, vec: &mut Vec<Beam>) {
            if beam.col > 0 {
                vec.push(Beam::new(Dir::L, beam.row, beam.col - 1));
            }
        }

        fn move_right(world: &World, beam: &Beam, vec: &mut Vec<Beam>) {
            if beam.col < world.cols - 1 {
                vec.push(Beam::new(Dir::R, beam.row, beam.col + 1));
            }
        }

        match beam.dir {
            Dir::U => match self.map[beam.row][beam.col] {
                Block::Empty | Block::Vertical => {
                    move_up(self, beam, &mut res);
                }
                Block::Horizontal => {
                    move_left(self, beam, &mut res);
                    move_right(self, beam, &mut res);
                }
                Block::RightUp => {
                    move_right(self, beam, &mut res);
                }
                Block::RightDown => {
                    move_left(self, beam, &mut res);
                }
            },
            Dir::D => match self.map[beam.row][beam.col] {
                Block::Empty | Block::Vertical => {
                    move_down(self, beam, &mut res);
                }
                Block::Horizontal => {
                    move_left(self, beam, &mut res);
                    move_right(self, beam, &mut res);
                }
                Block::RightUp => {
                    move_left(self, beam, &mut res);
                }
                Block::RightDown => {
                    move_right(self, beam, &mut res);
                }
            },
            Dir::L => match self.map[beam.row][beam.col] {
                Block::Empty | Block::Horizontal => {
                    move_left(self, beam, &mut res);
                }
                Block::Vertical => {
                    move_up(self, beam, &mut res);
                    move_down(self, beam, &mut res);
                }
                Block::RightUp => {
                    move_down(self, beam, &mut res);
                }
                Block::RightDown => {
                    move_up(self, beam, &mut res);
                }
            },
            Dir::R => match self.map[beam.row][beam.col] {
                Block::Empty | Block::Horizontal => {
                    move_right(self, beam, &mut res);
                }
                Block::Vertical => {
                    move_up(self, beam, &mut res);
                    move_down(self, beam, &mut res);
                }
                Block::RightUp => {
                    move_up(self, beam, &mut res);
                }
                Block::RightDown => {
                    move_down(self, beam, &mut res);
                }
            },
        }
        res
    }
}

fn parse_input() -> World {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let map = lines
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .into_iter()
                .map(|c| match c {
                    '.' => Block::Empty,
                    '-' => Block::Horizontal,
                    '|' => Block::Vertical,
                    '/' => Block::RightUp,
                    '\\' => Block::RightDown,
                    _ => panic!("Unknown symbol"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    World::new(map)
}

fn energy_from(world: &World, start_beam: Beam) -> usize {
    let mut beams = vec![start_beam];

    let mut covered = HashSet::new();
    covered.insert(start_beam);
    let mut covered_len = covered.len();

    loop {
        beams = beams.iter().flat_map(|beam| world.mv_bean(beam)).collect();
        beams.iter().for_each(|beam| {
            covered.insert(*beam);
        });
        if covered_len == covered.len() {
            break;
        } else {
            covered_len = covered.len();
        }
    }

    let covered = covered
        .into_iter()
        .map(|b| (b.row, b.col))
        .collect::<HashSet<_>>()
        .len();

    covered
}

fn main() {
    let world = parse_input();

    // First part
    let start_beam = Beam::new(Dir::R, 0, 0);
    let energy = energy_from(&world, start_beam);
    println!("Energy: {}", energy);

    // Second part
    let mut start_beams = Vec::new();
    (0..world.rows).for_each(|r| {
        start_beams.push(Beam::new(Dir::L, r, 0));
        start_beams.push(Beam::new(Dir::R, r, world.cols - 1));
    });
    (0..world.cols).for_each(|c| {
        start_beams.push(Beam::new(Dir::U, world.cols - 1, c));
        start_beams.push(Beam::new(Dir::D, 0, c));
    });
    let energy = start_beams
        .par_iter()
        .map(|&b| energy_from(&world, b))
        .max()
        .unwrap();
    println!("Energy: {}", energy);
}
