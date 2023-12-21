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

    fn move_up(&self, world: &World, res: &mut Vec<Path>, best: &mut HashMap<Pos, usize>) {
        if self.pos.row > 0 {
            let cost = self.cost + world.map[self.pos.row - 1][self.pos.col];
            let row = self.pos.row - 1;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::U, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.row > 1 {
            let cost = self.cost
                + world.map[self.pos.row - 1][self.pos.col]
                + world.map[self.pos.row - 2][self.pos.col];
            let row = self.pos.row - 2;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::U, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.row > 2 {
            let cost = self.cost
                + world.map[self.pos.row - 1][self.pos.col]
                + world.map[self.pos.row - 2][self.pos.col]
                + world.map[self.pos.row - 3][self.pos.col];
            let row = self.pos.row - 3;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::U, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
    }

    fn move_down(&self, world: &World, res: &mut Vec<Path>, best: &mut HashMap<Pos, usize>) {
        if self.pos.row < world.rows - 1 {
            let cost = self.cost + world.map[self.pos.row + 1][self.pos.col];
            let row = self.pos.row + 1;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::D, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.row < world.rows - 2 {
            let cost = self.cost
                + world.map[self.pos.row + 1][self.pos.col]
                + world.map[self.pos.row + 2][self.pos.col];
            let row = self.pos.row + 2;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::D, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.row < world.rows - 3 {
            let cost = self.cost
                + world.map[self.pos.row + 1][self.pos.col]
                + world.map[self.pos.row + 2][self.pos.col]
                + world.map[self.pos.row + 3][self.pos.col];
            let row = self.pos.row + 3;
            let col = self.pos.col;
            let path = Path::new(row, col, Dir::D, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
    }

    fn move_left(&self, world: &World, res: &mut Vec<Path>, best: &mut HashMap<Pos, usize>) {
        if self.pos.col > 0 {
            let cost = self.cost + world.map[self.pos.row][self.pos.col - 1];
            let row = self.pos.row;
            let col = self.pos.col - 1;
            let path = Path::new(row, col, Dir::L, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.col > 1 {
            let cost = self.cost
                + world.map[self.pos.row][self.pos.col - 1]
                + world.map[self.pos.row][self.pos.col - 2];
            let row = self.pos.row;
            let col = self.pos.col - 2;
            let path = Path::new(row, col, Dir::L, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.col > 2 {
            let cost = self.cost
                + world.map[self.pos.row][self.pos.col - 1]
                + world.map[self.pos.row][self.pos.col - 2]
                + world.map[self.pos.row][self.pos.col - 3];
            let row = self.pos.row;
            let col = self.pos.col - 3;
            let path = Path::new(row, col, Dir::L, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
    }

    fn move_right(&self, world: &World, res: &mut Vec<Path>, best: &mut HashMap<Pos, usize>) {
        if self.pos.col < world.cols - 1 {
            let cost = self.cost + world.map[self.pos.row][self.pos.col + 1];
            let row = self.pos.row;
            let col = self.pos.col + 1;
            let path = Path::new(row, col, Dir::R, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.col < world.cols - 2 {
            let cost = self.cost
                + world.map[self.pos.row][self.pos.col + 1]
                + world.map[self.pos.row][self.pos.col + 2];
            let row = self.pos.row;
            let col = self.pos.col + 2;
            let path = Path::new(row, col, Dir::R, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
        if self.pos.col < world.cols - 3 {
            let cost = self.cost
                + world.map[self.pos.row][self.pos.col + 1]
                + world.map[self.pos.row][self.pos.col + 2]
                + world.map[self.pos.row][self.pos.col + 3];
            let row = self.pos.row;
            let col = self.pos.col + 3;
            let path = Path::new(row, col, Dir::R, cost);
            let old_cost = best.get(&path.pos);
            if old_cost.is_none() || old_cost.is_some_and(|old| *old > cost) {
                best.insert(path.pos, path.cost);
                res.push(path);
            }
        }
    }

    fn make_moves(&self, world: &World, best: &mut HashMap<Pos, usize>) -> Vec<Path> {
        let mut res = Vec::new();
        let dir = self.pos.dir;
        match dir {
            Dir::U | Dir::D => {
                self.move_left(world, &mut res, best);
                self.move_right(world, &mut res, best);
            }
            Dir::L | Dir::R => {
                self.move_up(world, &mut res, best);
                self.move_down(world, &mut res, best);
            }
        }
        res
    }
}

fn shortest_path(world: &World) -> usize {
    let p1 = Path::new(0, 0, Dir::R, 0);
    let p2 = Path::new(0, 0, Dir::D, 0);
    let mut paths = vec![p1, p2];
    let mut best = HashMap::new();
    loop {
        paths = next_paths(&paths, world, &mut best);
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

fn next_paths(paths: &[Path], world: &World, best: &mut HashMap<Pos, usize>) -> Vec<Path> {
    paths
        .into_iter()
        .flat_map(|path| path.make_moves(world, best))
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
    let shortest_path = shortest_path(&world);
    println!("Shortest path: {}", shortest_path);
}
