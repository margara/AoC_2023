use std::{
    fs::File,
    io::{self, BufRead}, fmt::Display,
};

struct Lens {
    label: String,
    len: usize,
}

impl Lens {
    fn new(label: &str, len: usize) -> Self {
        let label = label.to_owned();
        Self { label, len }
    }

    fn change_len(&mut self, len: usize) {
        self.len = len;
    }
}

struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        let lenses = Vec::new();
        Self { lenses }
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }

    fn upsert_lens(&mut self, label: &str, len: usize) {
        if let Some(lens) = self.lenses.iter_mut().find(|lens| lens.label == label) {
            lens.change_len(len);
        } else {
            self.lenses.push(Lens::new(label, len));
        }
    }
}

impl Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lenses.iter().for_each(|l| {
            f.write_str(&l.label).unwrap();
            f.write_str("=").unwrap();
            f.write_fmt(format_args!("{} ", l.len)).unwrap();
        });
        f.write_str("\n").unwrap();
        Ok(())
    }
}

struct MyHashMap {
    boxes: [Box; 256],
}

impl Display for MyHashMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.boxes.iter().enumerate().for_each(|(c, b)| {
            f.write_fmt(format_args!("Box {}\n", c)).unwrap();
            b.fmt(f).unwrap();
        });
        Ok(())
    }
}

impl MyHashMap {
    fn new() -> Self {
        let boxes = std::array::from_fn(|_| Box::new());
        Self { boxes }
    }

    fn make_operation(&mut self, s: &str) {
        let op_index = s.find(|c| c == '-' || c == '=').unwrap();
        let label = &s[..op_index];
        let hash = hash(label) as usize;
        let mut it = s.chars();
        let op = it.nth(op_index).unwrap();
        match op {
            '-' => {
                self.boxes[hash].remove_lens(label);
            }
            '=' => {
                let len = it.next().unwrap();
                let len = len.to_string().parse::<usize>().unwrap();
                self.boxes[hash].upsert_lens(label, len);
            }
            _ => {
                panic!("Unknown operation {}", op);
            }
        }
    }

    fn compute_focus_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(c, b)| {
                let c = c + 1;
                let lenses = b
                    .lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_num, lens)| (lens_num+1) * lens.len)
                    .sum::<usize>();
                lenses * c
            })
            .sum::<usize>()
    }
}

fn parse_input() -> Vec<String> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    let strings = lines
        .flat_map(|line| {
            let line = line.unwrap();
            let line = line.split(",");
            line.map(|s| s.to_owned()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    strings
}

fn hash(s: &str) -> u32 {
    let mut val = 0;
    s.chars().map(|c| c as u32).for_each(|c| {
        val += c;
        val *= 17;
        val %= 256
    });

    val
}

fn main() {
    let input = parse_input();

    // First part
    let sum = input.iter().map(|s| hash(s)).sum::<u32>();
    println!("Sum: {}", sum);

    // Second part
    let mut map = MyHashMap::new();
    input.iter().for_each(|s| map.make_operation(s));
    println!("HashMap status: {}", map);
    let sum = map.compute_focus_power();
    println!("Sum: {}", sum);
}
