use std::{fs::File, io::{self, BufRead}, collections::{HashSet, HashMap}, cmp::min};

fn parse_input() -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let mut split = line.split(":");
        let mut split = split.nth(1).unwrap().split("|");
        let winning = split.next().unwrap().split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let game = split.next().unwrap().split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        (winning, game)
    }).collect::<Vec<_>>()
}

fn main() {
    // First part
    let input = parse_input();
    let sum = input.iter().map(|(win, game)| {
        win.intersection(game).collect::<Vec<_>>().len()
    }).map(|len| {
        if len == 0 {
            0
        } else {
            let len = len-1;
            2_u32.pow(len.try_into().unwrap())
        }
    })
    .sum::<u32>();

    println!("Sum: {}", sum);

    // Second part
    let mut count = (0..input.len()).map(|i| (i, 1)).collect::<HashMap<_,_>>();
    for card in 0..input.len() {
        let card_count = *count.get(&card).unwrap();
        let (win, game) = &input[card];
        let card_wins = win.intersection(&game).count();
        let start_id = card + 1;
        let end_id = min(start_id + card_wins, count.len());
        for id in start_id..end_id {
            *count.get_mut(&id).unwrap() += card_count;
        }
    }

    let sum = count.iter().map(|(_, c)| c).sum::<u32>();

    println!("Sum: {}", sum);
}
