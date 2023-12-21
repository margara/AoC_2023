use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    usize,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    U,
    D,
    R,
    L,
}

struct World {
    map: Vec<Vec<usize>>,
    rows: usize,
    cols: usize,
}

impl World {
    fn new(map: Vec<Vec<usize>>) -> Self {
        let rows = map.len();
        let cols = map.first().unwrap().len();
        Self { map, rows, cols }
    }
}

struct Path {
    pos: Pos,
    cost: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
    dir: Dir,
}

impl Path {
    fn new(row: usize, col: usize, dir: Dir, cost: usize) -> Self {
        Self {
            pos: Pos { row, col, dir },
            cost,
        }
    }

    fn move_up(
        &self,
        world: &World,
        res: &mut Vec<Path>,
        best: &mut HashMap<Pos, usize>,
        min_moves: usize,
        max_moves: usize,
    ) {
        (min_moves..max_moves + 1).for_each(|i| {
            if self.pos.row >= i {
                let cost = self.cost
                    + (1..i + 1)
                        .map(|j| world.map[self.pos.row - j][self.pos.col])
                        .sum::<usize>();
                let row = self.pos.row - i;
                let col = self.pos.col;
                let path = Path::new(row, col, Dir::U, cost);
                let old_cost = best.get(&path.pos);
                if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                    best.insert(path.pos, path.cost);
                    res.push(path);
                }
            }
        });
    }

    fn move_down(
        &self,
        world: &World,
        res: &mut Vec<Path>,
        best: &mut HashMap<Pos, usize>,
        min_moves: usize,
        max_moves: usize,
    ) {
        (min_moves..max_moves + 1).for_each(|i| {
            if self.pos.row < world.rows - i {
                let cost = self.cost
                    + (1..i + 1)
                        .map(|j| world.map[self.pos.row + j][self.pos.col])
                        .sum::<usize>();
                let row = self.pos.row + i;
                let col = self.pos.col;
                let path = Path::new(row, col, Dir::D, cost);
                let old_cost = best.get(&path.pos);
                if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                    best.insert(path.pos, path.cost);
                    res.push(path);
                }
            }
        });
    }

    fn move_left(
        &self,
        world: &World,
        res: &mut Vec<Path>,
        best: &mut HashMap<Pos, usize>,
        min_moves: usize,
        max_moves: usize,
    ) {
        (min_moves..max_moves + 1).for_each(|i| {
            if self.pos.col >= i {
                let cost = self.cost
                    + (1..i + 1)
                        .map(|j| world.map[self.pos.row][self.pos.col - j])
                        .sum::<usize>();
                let row = self.pos.row;
                let col = self.pos.col - i;
                let path = Path::new(row, col, Dir::L, cost);
                let old_cost = best.get(&path.pos);
                if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                    best.insert(path.pos, path.cost);
                    res.push(path);
                }
            }
        });
    }

    fn move_right(
        &self,
        world: &World,
        res: &mut Vec<Path>,
        best: &mut HashMap<Pos, usize>,
        min_moves: usize,
        max_moves: usize,
    ) {
        (min_moves..max_moves + 1).for_each(|i| {
            if self.pos.col < world.cols - i {
                let cost = self.cost
                    + (1..i + 1)
                        .map(|j| world.map[self.pos.row][self.pos.col + j])
                        .sum::<usize>();
                let row = self.pos.row;
                let col = self.pos.col + i;
                let path = Path::new(row, col, Dir::R, cost);
                let old_cost = best.get(&path.pos);
                if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                    best.insert(path.pos, path.cost);
                    res.push(path);
                }
            }
        });
    }

    fn make_moves(
        &self,
        world: &World,
        best: &mut HashMap<Pos, usize>,
        min_moves: usize,
        max_moves: usize,
    ) -> Vec<Path> {
        let mut res = Vec::new();
        let dir = self.pos.dir;
        match dir {
            Dir::U | Dir::D => {
                self.move_left(world, &mut res, best, min_moves, max_moves);
                self.move_right(world, &mut res, best, min_moves, max_moves);
            }
            Dir::L | Dir::R => {
                self.move_up(world, &mut res, best, min_moves, max_moves);
                self.move_down(world, &mut res, best, min_moves, max_moves);
            }
        }
        res
    }
}

fn shortest_path(world: &World, min_moves: usize, max_moves: usize) -> usize {
    let p1 = Path::new(0, 0, Dir::R, 0);
    let p2 = Path::new(0, 0, Dir::D, 0);
    let mut paths = vec![p1, p2];
    let mut best = HashMap::new();
    loop {
        paths = next_paths(&paths, world, &mut best, min_moves, max_moves);
        if paths.is_empty() {
            break;
        }
    }

    best.into_iter()
        .filter(|(k, _v)| k.row == world.rows - 1 && k.col == world.cols - 1)
        .map(|(_k, v)| v)
        .min()
        .unwrap()
}

fn next_paths(
    paths: &[Path],
    world: &World,
    best: &mut HashMap<Pos, usize>,
    min_moves: usize,
    max_moves: usize,
) -> Vec<Path> {
    paths
        .into_iter()
        .flat_map(|path| path.make_moves(world, best, min_moves, max_moves))
        .collect()
}

fn parse_input() -> World {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let map = lines
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    World::new(map)
}

fn main() {
    let world = parse_input();

    // First part
    let res = shortest_path(&world, 1, 3);
    println!("Shortest path: {}", res);

    // Second part
    let res = shortest_path(&world, 4, 10);
    println!("Shortest path: {}", res);
}
