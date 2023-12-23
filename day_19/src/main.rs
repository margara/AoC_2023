use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, Hash)]
enum Cat {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy)]
enum Op {
    LT,
    GT,
}

struct Part {
    attrs: HashMap<Cat, usize>,
}

impl Part {
    fn new(attrs: HashMap<Cat, usize>) -> Self {
        Self { attrs }
    }

    fn rating(&self) -> usize {
        self.attrs.values().sum::<usize>()
    }
}

struct Constraint {
    cat: Cat,
    op: Op,
    val: usize,
    target: Target,
}

impl Constraint {
    fn new(cat: Cat, op: Op, val: usize, target: Target) -> Self {
        Self {
            cat,
            op,
            val,
            target,
        }
    }

    fn sat(&self, part: &Part) -> bool {
        match self.op {
            Op::LT => *part.attrs.get(&self.cat).unwrap() < self.val,
            Op::GT => *part.attrs.get(&self.cat).unwrap() > self.val,
        }
    }
}

enum Target {
    Accept,
    Reject,
    Workflow(String),
}

enum Rule {
    Constraint(Constraint),
    Target(Target),
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }
}

struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    fn new(workflows: Vec<Workflow>) -> Self {
        let workflows = workflows
            .into_iter()
            .map(|w| (w.name.clone(), w))
            .collect::<HashMap<_, _>>();

        Self { workflows }
    }

    fn process_part(&self, name: &str, part: &Part) -> bool {
        let workflow = self.workflows.get(name).unwrap();
        let mut it = workflow.rules.iter();
        loop {
            let rule = it.next().unwrap();
            match rule {
                Rule::Constraint(c) => {
                    if c.sat(part) {
                        match &c.target {
                            Target::Accept => {
                                return true;
                            }
                            Target::Reject => {
                                return false;
                            }
                            Target::Workflow(w) => {
                                return self.process_part(w, part);
                            }
                        }
                    }
                }
                Rule::Target(t) => match t {
                    Target::Accept => {
                        return true;
                    }
                    Target::Reject => {
                        return false;
                    }
                    Target::Workflow(w) => {
                        return self.process_part(&w, part);
                    }
                },
            }
        }
    }
}

fn parse_input() -> (System, Vec<Part>) {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut workflows = Vec::new();
    let mut parts = Vec::new();

    lines.for_each(|line| {
        let line = line.unwrap();
        if !line.is_empty() {
            if line.starts_with("{") {
                let line = line.replace("{", "").replace("}", "");
                let line = line.split(",");
                let part = line
                    .map(|c| {
                        let mut it = c.split("=");
                        let cat = it.next().unwrap();
                        let val = it.next().unwrap();
                        let cat = match cat {
                            "x" => Cat::X,
                            "m" => Cat::M,
                            "a" => Cat::A,
                            "s" => Cat::S,
                            _ => panic!("Unknown attribute"),
                        };
                        let val = val.parse::<usize>().unwrap();
                        (cat, val)
                    })
                    .collect::<HashMap<_, _>>();
                parts.push(Part::new(part));
            } else {
                let line = line.replace("}", "");
                let mut line = line.split("{");
                let name = line.next().unwrap();
                let rules = line.next().unwrap();
                let rules = rules.split(",");
                let rules = rules
                    .map(|rule| {
                        if rule.contains(":") {
                            let mut rule = rule.split(":");
                            let c = rule.next().unwrap();
                            let t = rule.next().unwrap();
                            let op = if c.contains(">") { Op::GT } else { Op::LT };
                            let mut c = if c.contains(">") {
                                c.split(">")
                            } else {
                                c.split("<")
                            };
                            let cat = c.next().unwrap();
                            let val = c.next().unwrap();
                            let cat = match cat {
                                "x" => Cat::X,
                                "m" => Cat::M,
                                "a" => Cat::A,
                                "s" => Cat::S,
                                _ => panic!("Unknown attribute"),
                            };
                            let val = val.parse::<usize>().unwrap();
                            let target = match t {
                                "A" => Target::Accept,
                                "R" => Target::Reject,
                                _ => Target::Workflow(t.to_string()),
                            };
                            Rule::Constraint(Constraint::new(cat, op, val, target))
                        } else {
                            match rule {
                                "A" => Rule::Target(Target::Accept),
                                "R" => Rule::Target(Target::Reject),
                                _ => Rule::Target(Target::Workflow(rule.to_string())),
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                let workflow = Workflow::new(name.to_string(), rules);
                workflows.push(workflow);
            }
        }
    });

    let system = System::new(workflows);
    (system, parts)
}

fn main() {
    let (system, parts) = parse_input();

    // First part
    let sum = parts
        .iter()
        .filter(|part| system.process_part("in", part))
        .map(|part| part.rating())
        .sum::<usize>();
    println!("Sum: {}", sum);
}
