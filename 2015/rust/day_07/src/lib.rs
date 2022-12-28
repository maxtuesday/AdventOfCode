use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum GateType {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    PASS,
}

impl FromStr for GateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::AND),
            "OR" => Ok(GateType::OR),
            "LSHIFT" => Ok(GateType::LSHIFT),
            "RSHIFT" => Ok(GateType::RSHIFT),
            "NOT" => Ok(GateType::NOT),
            _ => Err("Cannot parse GateType".to_string()),
        }
    }
}

// Types of gates:
// - AND, OR        (2 inputs)
// - LSHIFT. RSHIFT (2 inputs)
// - NOT            (1 input)
#[derive(Clone, Debug)]
struct Gate {
    inputs: Vec<String>,
    output: String,
    gate_type: GateType,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<&str>>();
        let maybe_gate_type = tokens[1].parse::<GateType>();
        let maybe_not_gate_type = tokens[0].parse::<GateType>();
        let output = tokens.last().unwrap().to_string();
        let (gate_type, inputs) = match (maybe_gate_type, maybe_not_gate_type) {
            (Ok(gate_type), _) => { // AND, OR, LSHIFT, RSHIFT
                (gate_type, vec![tokens[0].to_string(), tokens[2].to_string()])
            }
            (Err(_), Ok(gate_type)) => { // NOT Gate
                (gate_type, vec![tokens[1].to_string()])
            }
            (Err(_), Err(_)) => { // PASS Gate
                (GateType::PASS, vec![tokens[0].to_string()])
            }
        };
        Ok(Gate {
            inputs,
            output,
            gate_type,
        })
    }
}

// #[derive(Debug, Clone)]
// struct Wire {
//     signal: Option<u16>,
// }

pub fn process_part_1(input: &str) -> String {
    let mut wires: HashMap<String, Option<u16>> = HashMap::new();
    let mut gates = input.lines()
        .map(|line| line.parse::<Gate>().unwrap())
        .collect::<Vec<Gate>>();
    // dbg!(&gates);

    for gate in &gates {
        for gate_input in &gate.inputs {
            wires.insert(gate_input.to_string(), gate_input.parse::<u16>().ok());
        }
        wires.insert(gate.output.to_string(), None);
    }

    dbg!(&wires);
    dbg!(&gates);

    loop {
        let unresolved_gates = gates.iter()
            .filter(|gate| wires.get(gate.output.as_str()).is_none())
            .collect::<Vec<_>>();
        if unresolved_gates.len() == 0 {
            break;
        }

        dbg!(&unresolved_gates);

        for gate in unresolved_gates {
            let num_inputs = gate.inputs.len();
            let inputs = gate.inputs.iter()
                .filter_map(|input| *wires.get(input).unwrap())
                .collect::<Vec<_>>();
            if num_inputs == inputs.len() { // gate inputs are resolved and we can resolve the output
                wires.entry(gate.output.to_string())
                    .and_modify(|signal| {
                        *signal = match gate.gate_type {
                            GateType::AND => Some(inputs[0] & inputs[1]),
                            GateType::OR => Some(inputs[0] | inputs[1]),
                            GateType::LSHIFT => Some(inputs[0] << inputs[1]),
                            GateType::RSHIFT => Some(inputs[0] >> inputs[1]),
                            GateType::NOT => Some(!inputs[0]),
                            GateType::PASS => Some(inputs[0]),
                        }
                    });
            }
        }

        dbg!(&wires);
        todo!("In loop, break")
    }

    todo!()
}

pub fn process_part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(""), "");
    }
}
