use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Spring {
    O,
    D,
    U,
}

struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn combinations(&self) -> usize {
        let mut cache = HashMap::new();
        self.combinations_for_slice(&mut cache, true, &self.springs, 0, 0)
    }

    fn combinations_for_slice(
        &self,
        cache: &mut HashMap<(usize, usize), usize>,
        do_cache: bool,
        springs: &[Spring],
        cur_spring: usize,
        cur_group: usize,
    ) -> usize {
        if let Some(&count) = cache.get(&(cur_spring, cur_group)) {
            return count;
        }

        // Base cases
        // 1) End of sequence
        if cur_spring >= springs.len() {
            return if cur_group == self.groups.len() { 1 } else { 0 };
        }
        // 2) End of groups
        if cur_group == self.groups.len() {
            return if (cur_spring..springs.len())
                .map(|i| springs[i])
                .any(|s| s == Spring::D)
            {
                0
            } else {
                1
            };
        }

        let count = match springs[cur_spring] {
            Spring::O => {
                // Skip
                self.combinations_for_slice(cache, true, springs, cur_spring + 1, cur_group)
            }
            Spring::D => {
                let group_len = self.groups[cur_group];
                // Not enough springs left
                if springs.len() - cur_spring < group_len {
                    0
                }
                // Not enough D or U to form a group
                else if (cur_spring..cur_spring + group_len)
                    .map(|i| springs[i])
                    .any(|s| s == Spring::O)
                {
                    0
                }
                // Too many D for this group
                else if cur_spring + group_len < self.springs.len()
                    && springs[cur_spring + group_len] == Spring::D
                {
                    0
                }
                // Otherwise, call recursively after consuming a group
                else {
                    self.combinations_for_slice(
                        cache,
                        true,
                        springs,
                        cur_spring + group_len + 1,
                        cur_group + 1,
                    )
                }
            }
            Spring::U => {
                // Call recursively twice (considering the position as operational and as damaged)
                let mut o_case = springs.to_vec();
                o_case[cur_spring] = Spring::O;
                let mut d_case = springs.to_vec();
                d_case[cur_spring] = Spring::D;
                self.combinations_for_slice(cache, false, &o_case, cur_spring, cur_group)
                    + self.combinations_for_slice(cache, false, &d_case, cur_spring, cur_group)
            }
        };

        if do_cache {
            cache.insert((cur_spring, cur_group), count);
        }
        
        count
    }
}

fn parse_input() -> Vec<Record> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            let springs = line.next().unwrap().to_string();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '.' => Spring::O,
                    '#' => Spring::D,
                    '?' => Spring::U,
                    _ => panic!("Unknown spring type"),
                })
                .collect::<Vec<_>>();

            let groups = line
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Record::new(springs, groups)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse_input();

    // First part
    let sum = input
        .iter()
        .map(|record| record.combinations())
        .sum::<usize>();
    println!("Sum: {}", sum);

    // Second part
    let sum = input
        .iter()
        .map(|record| {
            let mut springs = record.springs.clone();
            springs.push(Spring::U);
            springs = springs.repeat(5);
            springs.pop();
            let groups = record.groups.repeat(5);
            Record::new(springs, groups)
        })
        .map(|record| record.combinations())
        .sum::<usize>();
    println!("Sum: {}", sum);
}
