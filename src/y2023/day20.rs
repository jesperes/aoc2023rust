use std::collections::VecDeque;

use hashbrown::HashMap;

use crate::Solver;
pub struct Solution;
impl Solver<usize, usize> for Solution {
    fn solve(&self, input: &str) -> (usize, usize) {
        solve(input)
    }
}

#[derive(Debug, Clone)]
struct Signal {
    source: Option<String>,
    dest: String,
    value: SignalValue,
}

impl Signal {
    fn new(source: Option<String>, dest: &String, value: SignalValue) -> Self {
        Signal {
            source,
            dest: dest.to_string(),
            value,
        }
    }
}

type SignalValue = bool;
const HIGH: bool = true;
const LOW: bool = false;

#[derive(Debug, Hash, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjuction,
    Broadcast,
}

struct Module {
    module_type: ModuleType,
    module_name: String,
    outputs: Vec<String>,
    flip_flop_state: bool,
    conj_states: HashMap<String, SignalValue>,
}

struct Modules {
    module_map: HashMap<String, Module>,
    queue: VecDeque<Signal>,
    seen_rx_low: bool,
}

impl Modules {
    fn new() -> Self {
        Modules {
            module_map: HashMap::new(),
            queue: VecDeque::new(),
            seen_rx_low: false,
        }
    }

    fn add_module(&mut self, module_name: String, module_type: ModuleType, outputs: Vec<String>) {
        self.module_map.insert(
            module_name.to_string(),
            Module {
                module_name: module_name.to_string(),
                module_type,
                outputs,
                flip_flop_state: false,
                conj_states: HashMap::new(),
            },
        );
    }

    fn connect_conjunctions(&mut self) {
        let mut inputs_map: HashMap<String, Vec<String>> = HashMap::new();

        for (name, _mod_type) in &self.module_map {
            let module = self.module_map.get(name).unwrap();
            for output in &module.outputs {
                if output != "output" && output != "rx" {
                    let output_module = &self.module_map.get(output).unwrap();
                    if output_module.module_type == ModuleType::Conjuction {
                        let entry = inputs_map
                            .entry(output_module.module_name.to_string())
                            .or_default();
                        entry.push(name.to_string());
                    }
                }
            }
        }

        for (name, inputs) in inputs_map {
            for input_module in inputs {
                self.module_map
                    .get_mut(&name)
                    .unwrap()
                    .conj_states
                    .insert(input_module, false);
            }
        }
    }

    fn push_button(&mut self) -> usize {
        let mut num_high_signals = 0;
        let mut num_low_signals = 0;

        for _ in 0..1000 {
            self.queue
                .push_back(Signal::new(None, &"broadcaster".to_string(), false));

            while let Some(signal) = self.queue.pop_front() {
                if signal.value {
                    num_high_signals += 1;
                } else {
                    num_low_signals += 1;
                }
                self.process_signal(signal);
            }
        }

        num_high_signals * num_low_signals
    }

    fn process_signal(&mut self, signal: Signal) {
        use ModuleType::*;

        // Ignore signals sent to "output"
        if signal.dest == "output" {
            return;
        }

        if signal.dest == "rx" {
            if signal.value == LOW {
                self.seen_rx_low = true;
            }
            return;
        }

        // Update switches
        self.module_map
            .entry(signal.dest.to_string())
            .and_modify(|module| {
                if module.module_type == Conjuction {
                    // Track the state of incoming signals
                    module
                        .conj_states
                        .entry(signal.source.unwrap())
                        .insert(signal.value);
                } else if module.module_type == FlipFlop && signal.value != HIGH {
                    // if a flip-flop receives a high pulse, nothing happens, otherwise
                    // it switches between HIGH/LOW.
                    module.flip_flop_state = !module.flip_flop_state;
                }
            });

        let module = self.module_map.get(&signal.dest).unwrap();
        let output_signal = !module.conj_states.iter().all(|(_, state)| *state);

        let out_signals =
            module
                .outputs
                .iter()
                .filter_map(|output| match (&module.module_type, signal.value) {
                    (Broadcast, value) => {
                        Some(Signal::new(Some(module.module_name.clone()), output, value))
                    }
                    (FlipFlop, value) if value == HIGH => None,
                    (FlipFlop, value) if value == LOW => Some(Signal::new(
                        Some(module.module_name.clone()),
                        output,
                        module.flip_flop_state,
                    )),
                    (Conjuction, _) => Some(Signal::new(
                        Some(module.module_name.clone()),
                        output,
                        output_signal,
                    )),
                    _ => unreachable!(),
                });

        out_signals.for_each(|signal| self.queue.push_back(signal));
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut modules = Modules::new();

    for line in input.lines() {
        use ModuleType::*;
        let mut elems = line
            .split(|c| c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        let name: String = elems.next().unwrap();
        let outputs: Vec<String> = elems.skip(1).collect::<Vec<_>>();

        match name.chars().next().unwrap() {
            'b' => modules.add_module(name.to_string(), Broadcast, outputs),
            '%' => modules.add_module(name[1..].to_string(), FlipFlop, outputs),
            '&' => modules.add_module(name[1..].to_string(), Conjuction, outputs),
            _ => unreachable!(),
        };
    }
    modules.connect_conjunctions();
    let p1 = modules.push_button();
    let p2 = solve_p2(input);
    (p1, p2)
}

fn solve_p2(input: &str) -> usize {
    let mut graph = HashMap::new();

    // Clever solution stolen shamelessly from the megathread. Originally
    // written in Python, translated to Rust here.
    // https://www.reddit.com/r/adventofcode/comments/18mmfxb/comment/ke5sgxs/?utm_source=share&utm_medium=web2x&context=3
    input
        .lines()
        .map(|line| {
            line.split(|c| " ->,".contains(c))
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            graph.insert(line[0].to_string(), line[1..].to_vec());
        });

    let mut res = Vec::new();
    let initvec = graph.get("broadcaster").unwrap();

    for m in initvec {
        let mut m2 = m;
        let mut bin = String::new();

        // Decode chains of flip flops as bits in an integer, then
        // compute the lcm of them.
        loop {
            let key = format!("%{m2}");
            let g = graph.get(&key).unwrap();
            let key0 = format!("%{}", g[0]);
            bin.insert(
                0,
                if g.len() == 2 || !graph.contains_key(&key0) {
                    '1'
                } else {
                    '0'
                },
            );

            // Flip-flops that link to a conjunction are ones everything else is
            // a zero
            let mut nextl = Vec::new();
            let key1 = format!("%{m2}");
            for next in graph.get(&key1).unwrap() {
                let next1 = format!("%{next}");
                if graph.contains_key(&next1) {
                    nextl.push(next);
                }
            }

            if nextl.is_empty() {
                break;
            }

            m2 = nextl[0];
        }

        res.push(usize::from_str_radix(bin.as_str(), 2).unwrap());
    }

    res.iter().fold(1, |a, b| num::integer::lcm(a, *b))
}
