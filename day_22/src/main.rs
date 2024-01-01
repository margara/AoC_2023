use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Brick {
    blocks: Vec<(usize, usize, usize)>,
    min_z: usize,
}

impl Brick {
    fn new(init: (usize, usize, usize), end: (usize, usize, usize)) -> Self {
        let blocks = if init.0 != end.0 {
            (init.0..end.0 + 1).map(|x| (x, init.1, init.2)).collect()
        } else if init.1 != end.1 {
            (init.1..end.1 + 1).map(|y| (init.0, y, init.2)).collect()
        } else if init.2 != end.2 {
            (init.2..end.2 + 1).map(|z| (init.0, init.1, z)).collect()
        } else {
            vec![(init.0, init.1, init.2)]
        };

        let min_z = blocks.iter().map(|(_x, _y, z)| *z).min().unwrap();

        Self { blocks, min_z }
    }

    fn can_move_down(&self, others: &[Brick]) -> bool {
        self.min_z > 1
            && self.blocks.iter().all(|&(x, y, z)| {
                let block = (x, y, z - 1);
                others.iter().all(|other_brick| {
                    other_brick
                        .blocks
                        .iter()
                        .all(|&other_block| other_block != block)
                })
            })
    }

    fn move_down(&mut self) {
        self.blocks.iter_mut().for_each(|(_x, _y, z)| *z -= 1);
        self.min_z -= 1;
    }

    fn holds(&self, other: &Brick) -> bool {
        self.blocks.iter().any(|&(x, y, z)| {
            other.blocks.iter().any(|&(other_x, other_y, other_z)| {
                other_x == x && other_y == y && other_z == z + 1
            })
        })
    }
}

fn move_down(bricks: &mut [Brick]) {
    loop {
        let mut something_changed = false;
        bricks.sort_by_key(|b| b.min_z);
        for i in 0..bricks.len() {
            let b = &bricks[i];
            let others = &bricks[0..i];
            if b.can_move_down(others) {
                something_changed = true;
                let b = &mut bricks[i];
                b.move_down();
            }
        }
        if !something_changed {
            break;
        }
    }
}

fn holds_relation(bricks: &[Brick]) -> HashMap<usize, Vec<usize>> {
    (0..bricks.len())
        .map(|i| {
            let holds = (i + 1..bricks.len())
                .filter(|&j| bricks[i].holds(&bricks[j]))
                .collect_vec();
            (i, holds)
        })
        .collect::<HashMap<_, _>>()
}

fn held_by_relation(bricks: &[Brick]) -> HashMap<usize, Vec<usize>> {
    (0..bricks.len())
        .map(|i| {
            let held_by = (0..i)
                .filter(|&j| bricks[j].holds(&bricks[i]))
                .collect_vec();
            (i, held_by)
        })
        .collect::<HashMap<_, _>>()
}

fn parse_input() -> Vec<Brick> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split("~");

            let start = line.next().unwrap();
            let start = start.split(",");
            let start: (usize, usize, usize) = start
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            let end = line.next().unwrap();
            let end = end.split(",");
            let end: (usize, usize, usize) = end
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            Brick::new(start, end)
        })
        .collect_vec()
}

fn chain_remove(
    b: usize,
    holds_rel: &HashMap<usize, Vec<usize>>,
    held_by_rel: &HashMap<usize, Vec<usize>>,
) -> usize {
    let mut all_removed: HashSet<usize> = HashSet::new();
    let mut new_removed = holds_rel
        .get(&b)
        .unwrap()
        .iter()
        .filter(|brick| held_by_rel.get(brick).unwrap().len() == 1)
        .cloned()
        .collect::<HashSet<_>>();

    loop {
        if new_removed.is_empty() {
            break;
        } else {
            all_removed.extend(new_removed.iter());
        }
        new_removed = new_removed.iter().flat_map(|nr| {
            holds_rel
                .get(nr)
                .unwrap()
                .iter()
                .filter(|brick| {
                    held_by_rel
                        .get(brick)
                        .unwrap()
                        .iter()
                        .filter(|b| !all_removed.contains(b))
                        .count()
                        == 0
                })
                .cloned()
                .collect::<HashSet<_>>()
        }).collect();
    }

    all_removed.len()
}

fn main() {
    let mut bricks = parse_input();

    // Part one
    move_down(&mut bricks);
    let holds_rel = holds_relation(&bricks);
    let held_by_rel = held_by_relation(&bricks);

    bricks.iter().for_each(|b| println!("{:?}", b));
    println!("Holds: {:?}", holds_rel);
    println!("Held by: {:?}", held_by_rel);

    let res = holds_rel
        .iter()
        .filter(|(_, holds_set)| {
            holds_set.len() == 0
                || holds_set
                    .iter()
                    .all(|held| held_by_rel.get(held).unwrap().len() > 1)
        })
        .count();
    println!("Bricks to disintegrate: {}", res);

    // Part two
    let sum = (0..bricks.len()).map(|b| {
        let count = chain_remove(b, &holds_rel, &held_by_rel);
        println!("Brick: {}, removed: {}", b, count);
        count
    }).sum::<usize>();

    println!("Sum: {}", sum);
}
