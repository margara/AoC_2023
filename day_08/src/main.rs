use std::{fs::File, io::{self, BufRead}, collections::HashMap};

fn parse_input() -> (String, HashMap<String, (String, String)>) {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    let directions = lines.next().unwrap().unwrap();
    lines.next();

    let m = lines.map(|line| {
        let line = line.unwrap();
        let line = line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let mut line = line.split_whitespace();
        let source = line.next().unwrap().to_string();
        let left = line.next().unwrap().to_string();
        let right = line.next().unwrap().to_string();
        (source, (left, right))
    }).collect::<HashMap<_,_>>();

    (directions, m)
}

fn main() {
    let (directions, m) = parse_input();

    let mut count = 0;
    let mut source = String::from("AAA");
    loop {
        let d = directions.chars().nth(count % directions.len()).unwrap();
        let (left, right) = m.get(&source).unwrap();
        match d {
            'L' => {
                source = left.clone();
            }
            'R' => {
                source = right.clone();
            }
            _ => panic!("Unknown direction")
        }
        count += 1;
        if source == "ZZZ" {
            break;
        }
    }
    println!("Count: {}", count);

    let mut count = 0;
    let mut sources = m.keys().filter(|k| k.ends_with("A")).collect::<Vec<_>>();
    loop {
        let d = directions.chars().nth(count % directions.len()).unwrap();
        for i in 0..sources.len() {
            let mut source = *sources.get(i).unwrap();
            let (left, right) = m.get(source).unwrap();
            match d {
                'L' => {
                    source = left;
                }
                'R' => {
                    source = right;
                }
                _ => panic!("Unknown direction")
            }
            sources[i] = source;
        }
        count += 1;
        if sources.iter().all(|s| s.ends_with("Z")) {
            break;
        }
    }
    println!("Count: {}", count);

}
