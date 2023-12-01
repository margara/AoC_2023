use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    const RADIX: u32 = 10;
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let sum = lines.map(|line| {
        let line = line.unwrap();
        let digits = line.chars().into_iter()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(RADIX).unwrap())
            .collect::<Vec<_>>();
        digits.first().unwrap()*10 + digits.last().unwrap()
    }).sum::<u32>();

    // First part
    println!("Sum: {}", sum);

    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let sum = lines.map(|line| {
        let line = line.unwrap();
        let line = line
            .replace("one", "oonee")
            .replace("two", "ttwoo")
            .replace("three", "tthreee")
            .replace("four", "ffourr")
            .replace("five", "ffivee")
            .replace("six", "ssixx")
            .replace("seven", "ssevenn")
            .replace("eight", "eeightt")
            .replace("nine", "nninee")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

        let digits = line.chars().into_iter()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(RADIX).unwrap())
            .collect::<Vec<_>>();
        digits.first().unwrap()*10 + digits.last().unwrap()
    }).sum::<u32>();

    // Second part
    println!("Sum: {}", sum);
}
