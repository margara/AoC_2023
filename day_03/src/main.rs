use std::{fs::File, io::{self, BufRead}, collections::{HashSet, HashMap}};

#[derive(Debug, Clone)]
struct Number {
    line: usize,        // Line (first line has value 1)
    col_start: usize,   // Start column (included, columns start from 1)
    col_end: usize,     // End column (excluded, columns start from 1)
    val: u32,           // Value
}

impl Number {
    fn new(line: usize, col_start: usize, col_end: usize, val: u32) -> Self { 
        Self { line, col_start, col_end, val } 
    }
}

fn read_symbols() -> HashSet<(usize, usize)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut line_num = 1;
    lines.map(|line| {
        let line = line.unwrap();
        let line_set = line.char_indices()
            .filter(|(_i, c)| {
                !c.is_ascii_digit() && *c != '.'
            }).map(|(i, _c)| {
                (line_num, i+1)
            }).collect::<HashSet<_>>();
        line_num = line_num + 1;
        
        line_set
    }).flatten().collect()
}

fn read_gears() -> Vec<(usize, usize)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut line_num = 1;
    lines.map(|line| {
        let line = line.unwrap();
        let line_set = line.char_indices()
            .filter(|(_i, c)| {
                *c == '*'
            }).map(|(i, _c)| {
                (line_num, i+1)
            }).collect::<HashSet<_>>();
        line_num = line_num + 1;
        
        line_set
    }).flatten().collect()
}

fn read_numbers() -> HashMap<usize, Vec<Number>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut line_num = 1;
    lines.map(|line| {
        let line = line.unwrap();
        let line = line.as_bytes();
        let mut v = Vec::new();
        let mut num_vec = Vec::new();
        for i in 0..line.len() {
            if line[i].is_ascii_digit() {
                num_vec.push(line[i]);
                if i == line.len()-1 {
                    let val = String::from_utf8(num_vec.clone()).unwrap().parse::<u32>().unwrap();
                    let num = Number::new(line_num, i+2-num_vec.len(), i+2, val);
                    v.push(num);
                    num_vec.clear();
                }
            } else if !num_vec.is_empty() {
                let val = String::from_utf8(num_vec.clone()).unwrap().parse::<u32>().unwrap();
                let num = Number::new(line_num, i-num_vec.len()+1, i+1, val);
                v.push(num);
                num_vec.clear();
            }
        }
        let res = (line_num, v);
        line_num = line_num + 1;

        res
    }).collect()
}

fn find_adjacent_numbers(gear: &(usize, usize), numbers: &HashMap<usize, Vec<Number>>) -> Vec<Number> {
    let (line, col) = gear;
    let mut res: Vec<Number> = Vec::new();

    let prev_line = numbers.get(&(line-1)).unwrap().iter().filter(|n| {
        *col >= n.col_start-1 && *col <= n.col_end
    }).cloned().collect::<Vec<_>>();
    let same_line = numbers.get(&line).unwrap().iter().filter(|n| {
        *col == n.col_start-1 || *col == n.col_end
    }).cloned().collect::<Vec<_>>();
    let next_line = numbers.get(&(line+1)).unwrap().iter().filter(|n| {
        *col >= n.col_start-1 && *col <= n.col_end
    }).cloned().collect::<Vec<_>>();

    res.extend(prev_line.into_iter());
    res.extend(same_line.into_iter());
    res.extend(next_line.into_iter());

    res
}

fn main() {
    let numbers = read_numbers();
    let symbols = read_symbols();

    // First part
    let sum = numbers.iter()
        .map(|(_, v)| v)
        .flatten()
        .filter(|n| {
            let same_line = symbols.contains(&(n.line, n.col_start-1)) || symbols.contains(&(n.line, n.col_end));
            let adjacent_lines = (n.col_start-1..n.col_end+1).any(|col| {
                symbols.contains(&(n.line-1, col)) ||
                symbols.contains(&(n.line+1, col))
            });
    
            same_line || adjacent_lines
        }).map(|n| n.val)
        .sum::<u32>();

    println!("Sum: {}", sum);

    // Second part
    let gears = read_gears();
    let sum = gears.iter()
        .map(|g| find_adjacent_numbers(g, &numbers))
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.iter().map(|n| n.val).product::<u32>())
        .sum::<u32>();

    println!("Sum: {}", sum);
    
}
