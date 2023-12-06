use std::{fs::File, io::{self, BufRead}};

fn beat_record(time: u64, distance: u64, press_time: u64) -> bool {
    let run_time = time - press_time;
    let speed = press_time;
    let my_distance = speed * run_time;
    my_distance > distance
}

fn parse_input() -> Vec<(u64, u64)> {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    
    let times = lines.next().unwrap().unwrap();
    let times = times.split(":").nth(1).unwrap();
    let times = times.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let distances = lines.next().unwrap().unwrap();
    let distances = distances.split(":").nth(1).unwrap();
    let distances = distances.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>();

    times.iter().zip(distances.iter()).map(|(t, d)| (*t, *d)).collect()
}

fn parse_input2() -> (u64, u64) {
    let f = File::open("input/input.txt").unwrap();
    let mut lines = io::BufReader::new(f).lines();
    
    let time = lines.next().unwrap().unwrap();
    let time = time.split(":").nth(1).unwrap();
    let time = time.replace(" ", "").parse::<u64>().unwrap();

    let distance = lines.next().unwrap().unwrap();
    let distance = distance.split(":").nth(1).unwrap();
    let distance = distance.replace(" ", "").parse::<u64>().unwrap();

    (time, distance)
}

fn main() {
    // First part
    let input = parse_input();
    let res = input.iter().map(|(t, d)| {
        let press_times = 1 .. *t-1;
        press_times.filter(|press_time| beat_record(*t, *d, *press_time)).count()
    }).product::<usize>();
    println!("Result: {}", res);

    // Second part
    let (t, d) = parse_input2();
    let press_times = 1 .. t-1;
    let res = press_times.filter(|press_time| beat_record(t, d, *press_time)).count();
    println!("Result: {}", res);
}
