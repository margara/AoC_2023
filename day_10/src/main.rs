use std::{fs::File, io::{self, BufRead}, collections::HashMap};
use Pipe::{NS, EW, NE, NW, SE, SW, G, START};
use Dir::{N, S, E, W};
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    NS, EW, NE, NW, SE, SW, G, START
}

#[derive(Clone, Copy)]
enum Dir {
    N, S, E, W
}

fn move_n((x, y): (usize, usize)) -> ((usize, usize), Dir) { 
    ((x, y-1), N) 
}

fn move_s((x, y): (usize, usize)) -> ((usize, usize), Dir) { 
    ((x, y+1), S) 
}

fn move_w((x, y): (usize, usize)) -> ((usize, usize), Dir) { 
    ((x-1, y), W) 
}

fn move_e((x, y): (usize, usize)) -> ((usize, usize), Dir) { 
    ((x+1, y), E) 
}

fn next(pos: (usize, usize), pipe: Pipe, dir: Dir) -> Option<((usize, usize), Dir)> {
    match pipe {
       NS => {
        match dir {
            N => Some(move_n(pos)),
            S => Some(move_s(pos)),
            _ => None
        }
       }
       EW => {
        match dir {
            E => Some(move_e(pos)),
            W => Some(move_w(pos)),
            _ => None
        }
       }
       NE => {
        match dir {
            S => Some(move_e(pos)),
            W => Some(move_n(pos)),
            _ => None
        }
       }
       NW => {
        match dir {
            S => Some(move_w(pos)),
            E => Some(move_n(pos)),
            _ => None
        }
       }
       SE => {
        match dir {
            N => Some(move_e(pos)),
            W => Some(move_s(pos)),
            _ => None
        }
       }
       SW => {
        match dir {
            N => Some(move_w(pos)),
            E => Some(move_s(pos)),
            _ => None
        }
       }
       G => {
            None
       }
        _ => panic!("Unexpected ground")
    }
}

fn parse_input() -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut s = (0, 0);
    let input = lines.enumerate().map(|(y, line)| {
        let line = line.unwrap();
        let mut row = vec![G];
        let pipes = line.chars().enumerate().map(|(x, c)| { 
            match c {
                '|' => NS,
                '-' => EW,
                'L' => NE,
                'J' => NW,
                '7' => SW,
                'F' => SE,
                '.' => G,
                'S' => {
                    s = (x+1, y+1);
                    START
                },
                _ => panic!("Unknown pipe")
            }
        });
        row.extend(pipes);
        row.push(G);

        row
    }).collect::<Vec<_>>();
    
    let len = input.first().unwrap().len();
    let first_line = (0..len).map(|_| G).collect::<Vec<_>>();
    let last_line = first_line.clone();

    let mut all_lines = Vec::new();
    all_lines.push(first_line);
    all_lines.extend(input.into_iter());
    all_lines.push(last_line);

    (all_lines, s)
}

fn main() {
    let (pipes, s) = parse_input();

    // First part
    let mut paths = vec![
        Some(move_n(s)), 
        Some(move_s(s)), 
        Some(move_e(s)),
        Some(move_w(s))
    ];
    let mut steps = 1;
    loop {
        paths = paths.into_iter().map(|o| 
            o.map(|((x, y), dir)| {
                next((x,y), pipes[y][x], dir)
            }
        )).flatten().collect();
        if paths.iter().any(|o| o.is_some_and(|(pos, _)| pos == s)) {
            break;
        }
        steps += 1;
    }

    println!("Loop len: {}", steps);
    println!("Furthest point: {}", (steps+1)/2);

    // Second part
    let mut path = Some(move_n(s));
    let mut path_points = Vec::new();
    loop {
        let ((x, y), _) = path.unwrap();
        path_points.push((x, y));
        path = path.map(|((x, y), dir)| 
            next((x,y), pipes[y][x], dir)
        ).flatten();
        if let Some((pos, _)) = path {
            if pos == s {
                path_points.push(pos);
                break;
            }
        }
    }

    let path_points = path_points.into_iter()
        .into_group_map_by(|(_x, y)| *y)
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|(x, y)| (x, pipes[y][x]))
                    .collect::<HashMap<_,_>>()
            )
        }).collect::<HashMap<_,_>>();

    let x_max = pipes.first().unwrap().len();
    let mut inner_count = 0;
    path_points.into_iter().for_each(|(_y, points_dirs)| {
        let mut inside = false;
        for x in 0..x_max {
            if inside && !points_dirs.contains_key(&x) {
                inner_count += 1;
            } else if points_dirs.contains_key(&x) && (points_dirs[&x] == NS || points_dirs[&x] == NW || points_dirs[&x] == NE || points_dirs[&x] == START) {
                inside = !inside;
            }
        }
    });

    println!("Inner tiles: {}", inner_count);
}
