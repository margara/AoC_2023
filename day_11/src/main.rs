use std::{fs::File, io::{self, BufRead}, collections::HashSet, cmp::{min, max}};

#[derive(PartialEq, Eq, Clone)]
enum Space {
    GALAXY, EMPTY
}

fn distance(g1: &(usize, usize), g2: &(usize, usize), double_rows: &HashSet<usize>, double_columns: &HashSet<usize>, expansion: usize) -> usize {
    let mut count = 0;

    let xmin = min(g1.0, g2.0);
    let xmax = max(g1.0, g2.0);
    for x in xmin..xmax {
        count += if double_columns.contains(&x) { expansion } else { 1 };
    }

    let ymin = min(g1.1, g2.1);
    let ymax = max(g1.1, g2.1);
    for y in ymin..ymax {
        count += if double_rows.contains(&y) { expansion } else { 1 };
    }
    count
}

fn parse_input() -> (Vec<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    
    let mut double_rows = HashSet::new();
    let mut galaxies = Vec::new();

    let map = lines.enumerate().map(|(y, line)| {
        let line = line.unwrap();
        let line = line.chars().into_iter().enumerate().map(|(x, c)| {
            match c {
                '.' => Space::EMPTY,
                '#' => {
                    galaxies.push((x, y));
                    Space::GALAXY
                },
                _ => panic!("Unknown symbol")
            }
        }).collect::<Vec<_>>();
        if line.iter().all(|x| *x == Space::EMPTY) {
            double_rows.insert(y);
        }

        line
    }).collect::<Vec<_>>();

    let occupied_columns = galaxies.iter().map(|(x, _y)| *x).collect::<HashSet<_>>();
    let x_max = map.first().unwrap().len();
    let double_columns = (0..x_max).collect::<HashSet<_>>()
        .difference(&occupied_columns)
        .cloned()
        .collect::<HashSet<_>>();

    (galaxies, double_rows, double_columns)
}

fn main() {
    let (galaxies, double_rows, double_columns) = parse_input();
    
    // First part
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum += distance(&galaxies[i], &galaxies[j], &double_rows, &double_columns, 2);
        }
    }
    println!("Sum of distances: {}", sum);

    // Second part
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            sum += distance(&galaxies[i], &galaxies[j], &double_rows, &double_columns, 1_000_000);
        }
    }
    println!("Sum of distances: {}", sum);
}
