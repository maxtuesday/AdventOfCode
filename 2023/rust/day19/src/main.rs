use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day19.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Rating {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Rating {
    fn from(s: &str) -> Self {
        match s {
            "x" => Rating::ExtremelyCoolLooking,
            "m" => Rating::Musical,
            "a" => Rating::Aerodynamic,
            "s" => Rating::Shiny,
            c => unimplemented!("unknown rating charater: {c}"),
        }
    }
}

#[derive(Debug, Clone)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Destination {
    fn from(s: &str) -> Self {
        match s {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            name => Destination::Workflow(name.to_string()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(s: &str) -> Self {
        let (name, rest) = s.split_once('{').expect("should contain '{'");
        let (rules, _) = rest.split_once('}').expect("should end with '}'");
        let rules = rules.split(",").map(Rule::from).collect();
        Workflow {
            name: name.to_string(),
            rules,
        }
    }
}

#[derive(Debug)]
enum Qualifier {
    GreaterThan(u32),
    LessThan(u32),
}

#[derive(Debug)]
struct Condition {
    rating: Rating,
    qualifier: Qualifier,
}

#[derive(Debug)]
struct Rule {
    destination: Destination,
    condition: Option<Condition>,
}

impl Rule {
    fn from(s: &str) -> Self {
        // a<2006:qkq
        // A

        // check if there is a condition: does it contain a '>' or '<'
        if s.contains('>') || s.contains('<') {
            if let Some((rating, rest)) = s.split_once('>') {
                let (n, dest) = rest.split_once(':').expect("should contain a ':'");
                let n = n.parse().expect("should be a number");
                return Rule {
                    destination: Destination::from(dest),
                    condition: Some(Condition {
                        rating: Rating::from(rating),
                        qualifier: Qualifier::GreaterThan(n),
                    }),
                };
            } else if let Some((rating, rest)) = s.split_once('<') {
                let (n, dest) = rest.split_once(':').expect("should contain a ':'");
                let n = n.parse().expect("should be a number");
                return Rule {
                    destination: Destination::from(dest),
                    condition: Some(Condition {
                        rating: Rating::from(rating),
                        qualifier: Qualifier::LessThan(n),
                    }),
                };
            } else {
                unreachable!("must contain a '>' or '<' if str contains character");
            }
        } else {
            Rule {
                destination: Destination::from(s),
                condition: None,
            }
        }
    }
}

type Workflows = HashMap<String, Workflow>;
type Part = HashMap<Rating, u32>;

fn parse(input: &str) -> (Workflows, Vec<Part>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let workflows = sections[0]
        .lines()
        .map(|line| {
            let workflow = Workflow::from(line);
            (workflow.name.clone(), workflow)
        })
        .collect();
    let parts = sections[1]
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1]; // trim { }
            line.split(",")
                .map(|rating| {
                    let (r, n) = rating
                        .split_once('=')
                        .expect("ratings should be defined as char=num");
                    let n = n.parse().expect("valid number should follow rating");
                    (Rating::from(r), n)
                })
                .collect::<Part>()
        })
        .collect();
    (workflows, parts)
}

fn next_destination(part: &Part, workflow: &Workflow) -> Destination {
    for rule in workflow.rules.iter() {
        match &rule.condition {
            Some(condition) => {
                let Some(pv) = part.get(&condition.rating) else {
                    unimplemented!("rating not found in part");
                };
                match condition.qualifier {
                    Qualifier::GreaterThan(v) => {
                        if pv > &v {
                            return rule.destination.clone();
                        }
                    }
                    Qualifier::LessThan(v) => {
                        if pv < &v {
                            return rule.destination.clone();
                        }
                    }
                }
            }
            None => return rule.destination.clone(),
        }
    }
    return Destination::Rejected;
}

fn walk(part: &Part, workflows: &Workflows) -> Destination {
    let mut workflow = workflows
        .get(&String::from("in"))
        .expect("must contain a workflow named 'in'");
    loop {
        // determine which destination to send the part to
        let next = next_destination(part, workflow);
        match next {
            Destination::Accepted | Destination::Rejected => return next,
            Destination::Workflow(name) => {
                workflow = workflows.get(&name).expect("must contain workflow");
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);
    parts
        .iter()
        .filter(|part| matches!(walk(part, &workflows), Destination::Accepted))
        .map(|part| part.values().sum::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 19114);
    }
}
