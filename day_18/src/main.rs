use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Turn {
    NW,
    NE,
    SW,
    SE,
    WN,
    WS,
    EN,
    ES,
    N,
    S,
    W,
    E,
}

fn is_north(turn: &Turn) -> bool {
    *turn == Turn::N || *turn == Turn::NW || *turn == Turn::NE || *turn == Turn::EN || *turn == Turn::WN
}

fn is_south(turn: &Turn) -> bool {
    *turn == Turn::S || *turn == Turn::SW || *turn == Turn::SE || *turn == Turn::ES || *turn == Turn::WS
}

struct Command {
    dir: Dir,
    len: i64,
}

impl Command {
    fn new(dir: Dir, len: i64) -> Self {
        Self { dir, len }
    }
}

struct Plan {
    commands: Vec<Command>,
}

impl Plan {
    fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}

fn parse_input() -> Plan {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let commands = lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let dir = match line.next().unwrap() {
                "U" => Dir::North,
                "D" => Dir::South,
                "L" => Dir::West,
                "R" => Dir::East,
                _ => panic!("Unknown symbol"),
            };
            let len = line.next().unwrap();
            let len = len.to_owned().parse::<i64>().unwrap();

            Command::new(dir, len)
        })
        .collect::<Vec<_>>();

    Plan::new(commands)
}

fn parse_input2() -> Plan {
    let f = File::open("input/input_example.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let commands = lines
        .map(|line| {
            let line = line.unwrap();
            let line = line.split_whitespace();
            let color = line.last().unwrap();
            let len = i64::from_str_radix(&color[2..7], 16).unwrap();
            let dir = match color.chars().nth(7).unwrap() {
                '0' => Dir::East,
                '1' => Dir::South,
                '2' => Dir::West,
                '3' => Dir::North,
                _ => panic!("Parse error")
            };

            Command::new(dir, len)
        })
        .collect::<Vec<_>>();

    Plan::new(commands)
}

fn compute_border(plan: &Plan) -> Vec<(i64, i64, Turn)> {
    let mut pos = (0, 0, Turn::NE);
    let mut res = Vec::new();

    plan.commands
        .iter()
        .circular_tuple_windows()
        .for_each(|(command, next)| match command.dir {
            Dir::North => {
                (0..command.len).for_each(|i| {
                    pos.2 = Turn::N;
                    if i == command.len - 1 {
                        pos.2 = if next.dir == Dir::West {
                            Turn::NW
                        } else {
                            Turn::NE
                        };
                    }
                    pos.0 -= 1;
                    res.push(pos);
                });
            }
            Dir::South => {
                (0..command.len).for_each(|i| {
                    pos.2 = Turn::S;
                    if i == command.len - 1 {
                        pos.2 = if next.dir == Dir::West {
                            Turn::SW
                        } else {
                            Turn::SE
                        };
                    }
                    pos.0 += 1;
                    res.push(pos);
                });
            }
            Dir::West => {
                (0..command.len).for_each(|i| {
                    pos.2 = Turn::W;
                    if i == command.len - 1 {
                        pos.2 = if next.dir == Dir::North {
                            Turn::WN
                        } else {
                            Turn::WS
                        };
                    }
                    pos.1 -= 1;
                    res.push(pos);
                });
            }
            Dir::East => {
                (0..command.len).for_each(|i| {
                    pos.2 = Turn::E;
                    if i == command.len - 1 {
                        pos.2 = if next.dir == Dir::North {
                            Turn::EN
                        } else {
                            Turn::ES
                        };
                    }
                    pos.1 += 1;
                    res.push(pos);
                });
            }
        });

    res
}

fn fill(border: &[(i64, i64, Turn)]) -> usize {
    let mut count = 0;
    let border = border.iter().into_group_map_by(|(r, _c, _d)| *r);
    border.into_iter().for_each(|(_r, v)| {
        let v = v.iter().map(|(_r, c, d)| (c, d)).collect::<HashMap<_, _>>();
        let min = **v.keys().min().unwrap();
        let max = **v.keys().max().unwrap();

        let mut enter_north = true;
        for c in min..max+1 {
            if v.get(&c).is_some_and(|turn| is_north(turn)) {
                enter_north = true;
                break;
            }
            if v.get(&c).is_some_and(|turn| is_south(turn)) {
                enter_north = false;
                break;
            }
        }

        let mut inside = false;
        (min..max + 1).for_each(|c| {
            if v.get(&c).is_some_and(|turn| is_north(turn)) {
                inside = enter_north;
            } else if v.get(&c).is_some_and(|turn| is_south(turn)) {
                inside = !enter_north;
            }

            if inside || v.contains_key(&c) {
                count += 1;
            }
        });
    });

    count
}

fn main() {
    // First part
    let plan = parse_input();
    let border = compute_border(&plan);
    let total = fill(&border);
    println!("Border: {}; total: {}", border.len(), total);

    // Second part
    let plan = parse_input2();
    let border = compute_border(&plan);
    let total = fill(&border);
    println!("Border: {}; total: {}", border.len(), total);
}
