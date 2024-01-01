use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../../input/day20.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleKind {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Untyped,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: ModuleKind,
    outputs: Vec<String>,
}

fn parse_line(line: &str) -> Module {
    let (start, outputs) = line.split_once(" -> ").expect("line should contain '->'");
    let outputs = outputs.split(", ").map(str::to_string).collect::<Vec<_>>();
    let (name, kind) = match &start[0..1] {
        "%" => (start[1..].to_string(), ModuleKind::FlipFlop(false)),
        "&" => (
            start[1..].to_string(),
            ModuleKind::Conjunction(HashMap::new()),
        ),
        "b" => (start.to_string(), ModuleKind::Broadcaster),
        c => unimplemented!("unknown starting character: {c}"),
    };
    Module {
        name,
        kind,
        outputs,
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let module = parse_line(line);
            (module.name.clone(), module)
        })
        .collect::<HashMap<String, Module>>();

    // Key: Output
    // Value: List of inputs
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (input, module) in modules.iter() {
        for output in module.outputs.iter() {
            inputs
                .entry(output.clone())
                .and_modify(|inputs| {
                    inputs.push(input.clone());
                })
                .or_insert(Vec::from([input.clone()]));
        }
    }

    // For all Conjunction Modules, we need to set up all their inputs
    // To find the inputs for a module, we need to take each module and map over their outputs.
    // If an output is a Conjunction module,
    // then add the "source" as an input to the Conjunction module HashMap
    for (name, module) in modules.iter_mut() {
        match &module.kind {
            ModuleKind::Broadcaster => {}
            ModuleKind::FlipFlop(_) => {}
            ModuleKind::Untyped => {}
            ModuleKind::Conjunction(_) => {
                let module_inputs = inputs
                    .get(name)
                    .unwrap()
                    .iter()
                    .map(|input| (input.clone(), Pulse::Low))
                    .collect::<HashMap<String, Pulse>>();
                module.kind = ModuleKind::Conjunction(module_inputs);
            }
        }
    }
    modules
}

fn press_button(modules: &mut HashMap<String, Module>) -> (u32, u32) {
    #[derive(Debug)]
    struct Source {
        name: String,
        pulse: Pulse,
    }

    let broadcaster = modules.get(&String::from("broadcaster")).unwrap();
    let mut queue = VecDeque::from([(
        broadcaster.clone(),
        Source {
            name: String::from("button"),
            pulse: Pulse::Low,
        },
    )]);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    while let Some((module, source)) = queue.pop_front() {
        match source.pulse {
            Pulse::Low => low_pulse_count += 1,
            Pulse::High => high_pulse_count += 1,
        }
        match module.kind {
            ModuleKind::Broadcaster => {
                // There is a single broadcast module (named broadcaster).
                // When it receives a pulse, it sends the same pulse to all of its destination modules.
                for output in module.outputs.iter() {
                    let next_module = modules
                        .get(output)
                        .unwrap_or(&Module {
                            name: output.clone(),
                            kind: ModuleKind::Broadcaster,
                            outputs: vec![],
                        })
                        .clone();
                    queue.push_back((
                        next_module,
                        Source {
                            name: module.name.clone(),
                            pulse: Pulse::Low,
                        },
                    ));
                }
            }
            ModuleKind::FlipFlop(on) => {
                // Flip-flop modules (prefix %) are either on or off; they are initially off.
                // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                // However, if a flip-flop module receives a low pulse, it flips between on and off.
                // If it was on, it turns off and sends a low pulse.
                // If it was off, it turns on and sends a high pulse.
                match source.pulse {
                    Pulse::High => {}
                    Pulse::Low => {
                        let (next_state, next_pulse) = if on {
                            (ModuleKind::FlipFlop(false), Pulse::Low)
                        } else {
                            (ModuleKind::FlipFlop(true), Pulse::High)
                        };
                        for output in module.outputs.iter() {
                            let next_module = modules
                                .get(output)
                                .unwrap_or(&Module {
                                    name: output.clone(),
                                    kind: ModuleKind::Broadcaster,
                                    outputs: vec![],
                                })
                                .clone();
                            queue.push_back((
                                next_module,
                                Source {
                                    name: module.name.clone(),
                                    pulse: next_pulse.clone(),
                                },
                            ))
                        }
                        modules.entry(module.name.clone()).and_modify(|module| {
                            module.kind = next_state;
                        });
                    }
                }
            }
            ModuleKind::Conjunction(mut inputs) => {
                // Conjunction modules (prefix &)
                // remember the type of the most recent pulse received from each of their connected input modules;
                // they initially default to remembering a low pulse for each input.
                // When a pulse is received, the conjunction module first updates its memory for that input.
                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

                inputs.insert(source.name, source.pulse);
                let next_pulse = if inputs.values().all(|pulse| matches!(pulse, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for output in module.outputs.iter() {
                    let next_module = modules
                        .get(output)
                        .unwrap_or(&Module {
                            name: output.clone(),
                            kind: ModuleKind::Untyped,
                            outputs: vec![],
                        })
                        .clone();
                    queue.push_back((
                        next_module,
                        Source {
                            name: module.name.clone(),
                            pulse: next_pulse.clone(),
                        },
                    ))
                }

                modules.entry(module.name.clone()).and_modify(|module| {
                    module.kind = ModuleKind::Conjunction(inputs);
                });
            }
            ModuleKind::Untyped => {
                // do nothing, there are no rules for this type of module
            }
        }
    }
    (low_pulse_count, high_pulse_count)
}

fn part1(input: &str) -> u32 {
    let mut modules = parse(input);
    let (low_pulse_count, high_pulse_count) = (0..1000).fold((0, 0), |acc, _| {
        let (low_pulse_count, high_pulse_count) = press_button(&mut modules);
        (acc.0 + low_pulse_count, acc.1 + high_pulse_count)
    });
    low_pulse_count * high_pulse_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_part1_ex2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(input), 11687500);
    }
}
