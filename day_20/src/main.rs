use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Signal {
    L,
    H,
}

trait Module {
    fn name(&self) -> String;

    fn process_signal(&mut self, signal: Signal, in_module: &str) -> Option<Signal>;

    fn out_modules(&self) -> &[String];
}

struct Broadcast {
    name: String,
    out_modules: Vec<String>,
}

struct FlipFlop {
    name: String,
    out_modules: Vec<String>,
    state: bool,
}

struct Conjunction {
    name: String,
    out_modules: Vec<String>,
    state: HashMap<String, bool>,
}

impl Broadcast {
    fn new(name: &str, out_modules: &Vec<String>) -> Self {
        let name = name.to_owned();
        let out_modules = out_modules.iter().cloned().collect();
        Self { name, out_modules }
    }
}

impl Module for Broadcast {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn process_signal(&mut self, signal: Signal, _in_module: &str) -> Option<Signal> {
        Some(signal)
    }

    fn out_modules(&self) -> &[String] {
        &self.out_modules
    }
}

impl FlipFlop {
    fn new(name: &str, out_modules: &[String]) -> Self {
        let name = name.to_owned();
        let out_modules = out_modules.iter().cloned().collect();
        let state = false;
        Self {
            name,
            out_modules,
            state,
        }
    }
}

impl Module for FlipFlop {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn process_signal(&mut self, signal: Signal, _in_module: &str) -> Option<Signal> {
        match signal {
            Signal::L => {
                self.state = !self.state;
                if self.state {
                    Some(Signal::H)
                } else {
                    Some(Signal::L)
                }
            }
            Signal::H => None,
        }
    }

    fn out_modules(&self) -> &[String] {
        &self.out_modules
    }
}

impl Conjunction {
    fn new(name: &str, out_modules: &[String], in_modules: &[String]) -> Self {
        let name = name.to_owned();
        let out_modules = out_modules.iter().cloned().collect();
        let state = in_modules
            .iter()
            .map(|in_mod| (in_mod.clone(), false))
            .collect::<HashMap<_, _>>();

        Self {
            name,
            out_modules,
            state,
        }
    }
}

impl Module for Conjunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn process_signal(&mut self, signal: Signal, in_module: &str) -> Option<Signal> {
        *self.state.get_mut(in_module).unwrap() = match signal {
            Signal::H => true,
            Signal::L => false,
        };

        if self.state.iter().all(|(_in, s)| *s) {
            Some(Signal::L)
        } else {
            Some(Signal::H)
        }
    }

    fn out_modules(&self) -> &[String] {
        &self.out_modules
    }
}

struct System {
    modules: HashMap<String, Box<dyn Module>>,
}

#[derive(Debug)]
struct SignalPropagation {
    signal: Signal,
    sender: String,
    receiver: String,
}

impl SignalPropagation {
    fn new(signal: Signal, sender: String, receiver: String) -> Self {
        Self {
            signal,
            sender,
            receiver,
        }
    }
}

impl System {
    fn new(modules: Vec<Box<dyn Module>>) -> Self {
        let modules = modules
            .into_iter()
            .map(|module| (module.name(), module))
            .collect::<HashMap<_, _>>();
        Self { modules }
    }

    fn push_button(&mut self) -> (usize, usize) {
        let mut signals = vec![SignalPropagation::new(
            Signal::L,
            String::from("button"),
            String::from("broadcaster"),
        )];

        let mut num_low_signals = 1;
        let mut num_high_signals = 0;

        loop {
            // signals.iter().for_each(|s| println!("{:?}", s));

            signals = signals
                .into_iter()
                .flat_map(|signal_propagation| {
                    if let Some(rec_module) = self.modules.get_mut(&signal_propagation.receiver) {
                        let generated_signal = rec_module
                            .process_signal(signal_propagation.signal, &signal_propagation.sender);
                        if let Some(s) = generated_signal {
                            rec_module
                                .out_modules()
                                .iter()
                                .map(|new_receiver| {
                                    SignalPropagation::new(
                                        s,
                                        signal_propagation.receiver.clone(),
                                        new_receiver.clone(),
                                    )
                                })
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                })
                .collect();

            if signals.is_empty() {
                break;
            } else {
                num_low_signals += signals.iter().filter(|s| s.signal == Signal::L).count();
                num_high_signals += signals.iter().filter(|s| s.signal == Signal::H).count();
            }
        }

        (num_low_signals, num_high_signals)
    }

    fn push_button2(&mut self) -> bool {
        let mut signals = vec![SignalPropagation::new(
            Signal::L,
            String::from("button"),
            String::from("broadcaster"),
        )];

        loop {
            signals = signals
                .into_iter()
                .flat_map(|signal_propagation| {
                    if let Some(rec_module) = self.modules.get_mut(&signal_propagation.receiver) {
                        let generated_signal = rec_module
                            .process_signal(signal_propagation.signal, &signal_propagation.sender);
                        if let Some(s) = generated_signal {
                            rec_module
                                .out_modules()
                                .iter()
                                .map(|new_receiver| {
                                    SignalPropagation::new(
                                        s,
                                        signal_propagation.receiver.clone(),
                                        new_receiver.clone(),
                                    )
                                })
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                })
                .collect();

            if signals.is_empty() {
                break;
            } else if signals.iter().any(|s| s.receiver == "rx" && s.signal == Signal::L) {
                return true;
            }
        }

        false
    }
}

fn parse_input() -> System {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut flip_flops = HashSet::new();
    let mut conjunctions = HashSet::new();
    let mut out_modules_map = HashMap::new();

    lines.for_each(|line| {
        let line = line.unwrap();
        let mut line = line.split("->");

        let left = line.next().unwrap().trim();
        let name = if left.starts_with("%") || left.starts_with("&") {
            &left[1..]
        } else {
            left
        };

        let right = line.next().unwrap().trim();
        let right = right.split(",");
        let out_modules = right.map(|m| m.trim().to_string()).collect::<Vec<_>>();
        out_modules_map.insert(name.to_string(), out_modules);

        if left.starts_with("%") {
            flip_flops.insert(name.to_string());
        } else if left.starts_with("&") {
            conjunctions.insert(name.to_string());
        }
    });

    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    out_modules_map.iter().for_each(|(name, out_modules)| {
        if flip_flops.contains(name) {
            let m = FlipFlop::new(name, out_modules);
            modules.push(Box::new(m))
        } else if conjunctions.contains(name) {
            let in_modules = out_modules_map
                .iter()
                .filter(|(_k, v)| v.contains(name))
                .map(|(m, _mods)| m)
                .cloned()
                .collect::<Vec<_>>();
            let m = Conjunction::new(name, out_modules, &in_modules);
            modules.push(Box::new(m));
        } else {
            let m = Broadcast::new(name, out_modules);
            modules.push(Box::new(m));
        }
    });

    System::new(modules)
}

fn main() {
    // Part one
    let mut system = parse_input();
    let (num_low, num_high) = (0..1000)
        .map(|_| system.push_button())
        .reduce(|acc, el| (acc.0 + el.0, acc.1 + el.1))
        .unwrap();
    println!("Num signals (L, H, Prod): {} {} {}", num_low, num_high, num_low*num_high);

    // Part two
    let mut system = parse_input();
    let mut i: usize = 1;
    loop {
        if i % 1_000_000 == 0 {
            println!("Iteration: {}", i);
        }
        let res = system.push_button2();
        if res {
            println!("Found after {}", i);
            break;
        }
        i += 1;
    }
}
