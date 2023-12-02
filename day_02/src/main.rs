use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Hash)]
enum Colors {
    R, G, B
}

fn parse_input() -> HashMap<u32, Vec<HashMap<Colors, u32>>> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let mut split = line.split(":");
        let game_id = split.next().unwrap().replace("Game ", "").parse::<u32>().unwrap();
        let games = split.next().unwrap().split(";")
        .map(|game| {
            game.split(",").map(|el| {
                if el.ends_with("red") {
                    let val = el.replace("red", "").trim().parse::<u32>().unwrap();
                    (Colors::R, val)
                } 
                else if el.ends_with("green") {
                    let val = el.replace("green", "").trim().parse::<u32>().unwrap();
                     (Colors::G, val)
                } 
                else if el.ends_with("blue") {
                    let val = el.replace("blue", "").trim().parse::<u32>().unwrap();
                    (Colors::B, val)
                } 
                else {
                    panic!("Unexpecred color")
                }
            }).collect()
        }).collect();
        (game_id, games)
    }).collect()
}

fn main() {
    let games = parse_input();
    
    // First part
    let sum = games.iter().filter(|(_id, set)| {
        !(**set).iter().any(|el| {
            *el.get(&Colors::R).unwrap_or_else(|| &0) > 12 ||
            *el.get(&Colors::G).unwrap_or_else(|| &0) > 13 ||
            *el.get(&Colors::B).unwrap_or_else(|| &0) > 14
        })
    }).map(|(&id, _)| id)
    .sum::<u32>();

    println!("Sum: {}", sum);

    // Second part
    let sum = games.iter().map(|(_id, set)| {
        (*set).iter().map(|el| {
            (
                *el.get(&Colors::R).unwrap_or_else(|| &0),
                *el.get(&Colors::G).unwrap_or_else(|| &0),
                *el.get(&Colors::B).unwrap_or_else(|| &0)
            )
        }).reduce(|(r1, g1, b1), (r2, g2, b2)| {
            (max(r1, r2), max(g1, g2), max(b1, b2))
        }).unwrap()
    }).map(|(r, g, b)| r*g*b)
    .sum::<u32>();

    println!("Sum: {}", sum);
}
