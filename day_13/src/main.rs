use std::{
    cmp::min,
    fs::File,
    io::{self, BufRead},
    usize,
};

#[derive(PartialEq, Eq)]
enum Ground {
    A,
    R,
}

struct Map {
    num_rows: usize,
    num_columns: usize,
    map: Vec<Vec<Ground>>,
}

impl Map {
    fn new(num_rows: usize, num_columns: usize, map: Vec<Vec<Ground>>) -> Self {
        Self {
            num_rows,
            num_columns,
            map,
        }
    }

    fn test_mirror_after_col(&self, col: usize) -> bool {
        let len = min(col + 1, self.num_columns - col - 1);
        self.map
            .iter()
            .all(|row| (0..len).all(|c| row[col + c + 1] == row[col - c]))
    }

    fn test_mirror_after_col2(&self, col: usize) -> bool {
        let len = min(col + 1, self.num_columns - col - 1);
        self.map
            .iter()
            .map(|row| {
                (0..len)
                    .filter(|&c| row[col + c + 1] != row[col - c])
                    .count()
            })
            .sum::<usize>()
            == 1
    }

    fn test_mirror_after_row(&self, row: usize) -> bool {
        let len = min(row + 1, self.num_rows - row - 1);
        (0..len).all(|r| self.map[row + r + 1] == self.map[row - r])
    }

    fn test_mirror_after_row2(&self, row: usize) -> bool {
        let len = min(row + 1, self.num_rows - row - 1);
        (0..len)
            .map(|r| {
                self.map[row + r + 1]
                    .iter()
                    .zip(self.map[row - r].iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum::<usize>()
            == 1
    }

    fn compute_mirror_col(&self) -> Option<usize> {
        (0..self.num_columns - 1).find(|&c| self.test_mirror_after_col(c))
    }

    fn compute_mirror_col2(&self) -> Option<usize> {
        (0..self.num_columns - 1).find(|&c| self.test_mirror_after_col2(c))
    }

    fn compute_mirror_row(&self) -> Option<usize> {
        (0..self.num_rows - 1).find(|&r| self.test_mirror_after_row(r))
    }

    fn compute_mirror_row2(&self) -> Option<usize> {
        (0..self.num_rows - 1).find(|&r| self.test_mirror_after_row2(r))
    }

    fn summarize(&self) -> usize {
        let mirror_col = if let Some(col) = self.compute_mirror_col() {
            col + 1
        } else {
            0
        };
        let mirror_row = if let Some(row) = self.compute_mirror_row() {
            row + 1
        } else {
            0
        };
        let res = mirror_col + 100 * mirror_row;
        println!(">> col: {}, row: {}, res: {}", mirror_col, mirror_row, res);

        res
    }

    fn summarize2(&self) -> usize {
        let mirror_col = if let Some(col) = self.compute_mirror_col2() {
            col + 1
        } else {
            0
        };
        let mirror_row = if let Some(row) = self.compute_mirror_row2() {
            row + 1
        } else {
            0
        };
        let res = mirror_col + 100 * mirror_row;
        println!(">> col: {}, row: {}, res: {}", mirror_col, mirror_row, res);

        res
    }
}

fn parse_input() -> Vec<Map> {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    let mut res = Vec::new();
    let mut current_map: Vec<Vec<Ground>> = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.is_empty() {
                let map = std::mem::take(&mut current_map);
                let num_rows = map.len();
                let num_columns = map.first().unwrap().len();
                let m = Map::new(num_rows, num_columns, map);
                res.push(m);
            } else {
                let line = line
                    .chars()
                    .map(|c| match c {
                        '.' => Ground::A,
                        '#' => Ground::R,
                        _ => panic!("Unknown ground"),
                    })
                    .collect::<Vec<_>>();
                current_map.push(line);
            }
        } else {
            break;
        }
    }

    res
}

fn main() {
    // First part
    let sum = parse_input()
        .into_iter()
        .map(|m| m.summarize())
        .sum::<usize>();
    println!("Sum: {}", sum);

    // Second part
    let sum = parse_input()
        .into_iter()
        .map(|m| m.summarize2())
        .sum::<usize>();
    println!("Sum: {}", sum);
}
