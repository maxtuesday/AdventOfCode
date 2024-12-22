use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/day10.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos(i32, i32);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(graph: &Vec<Vec<char>>) -> Pos {
    for (r, row) in graph.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                return Pos(r as i32, c as i32);
            }
        }
    }
    unreachable!("we did not find a start location")
}

fn is_out_of_bounds(pos: &Pos, graph: &Vec<Vec<char>>) -> bool {
    pos.0 < 0 || pos.0 >= graph.len() as i32 || pos.1 < 0 || pos.1 >= graph[0].len() as i32
}

fn get_next(pos: &Pos, graph: &Vec<Vec<char>>, visited: &HashSet<Pos>) -> Option<Pos> {
    // Use current position to find next positions
    let next = match graph[pos.0 as usize][pos.1 as usize] {
        '|' => vec![Pos(pos.0 + 1, pos.1), Pos(pos.0 - 1, pos.1)],
        '-' => vec![Pos(pos.0, pos.1 + 1), Pos(pos.0, pos.1 - 1)],
        'L' => vec![Pos(pos.0 - 1, pos.1), Pos(pos.0, pos.1 + 1)],
        'J' => vec![Pos(pos.0 - 1, pos.1), Pos(pos.0, pos.1 - 1)],
        '7' => vec![Pos(pos.0 + 1, pos.1), Pos(pos.0, pos.1 - 1)],
        'F' => vec![Pos(pos.0 + 1, pos.1), Pos(pos.0, pos.1 + 1)],
        ch => unimplemented!("unrecognized character: {ch}"),
    };
    let next = next
        .into_iter()
        .filter(|pos| !is_out_of_bounds(pos, graph) && !visited.contains(pos))
        .collect::<Vec<_>>();
    let next = next.first();
    match next {
        Some(next) => Some(*next),
        None => None,
    }
}

fn get_start(pos: &Pos, graph: &Vec<Vec<char>>) -> Vec<Pos> {
    let up = !is_out_of_bounds(&Pos(pos.0 + 1, pos.1), graph)
        && matches!(graph[(pos.0 - 1) as usize][pos.1 as usize], '|' | '7' | 'F');
    let down = !is_out_of_bounds(&Pos(pos.0 - 1, pos.1), graph)
        && matches!(graph[(pos.0 + 1) as usize][pos.1 as usize], '|' | 'L' | 'J');
    let left = !is_out_of_bounds(&Pos(pos.0, pos.1 - 1), graph)
        && matches!(graph[pos.0 as usize][(pos.1 - 1) as usize], '-' | 'L' | 'F');
    let right = !is_out_of_bounds(&Pos(pos.0, pos.1 + 1), graph)
        && matches!(graph[pos.0 as usize][(pos.1 + 1) as usize], '-' | '7' | 'J');
    
    let mut next = Vec::new();
    if up {
        next.push(Pos(pos.0 - 1, pos.1))
    }
    if down {
        next.push(Pos(pos.0 + 1, pos.1))
    }
    if left {
        next.push(Pos(pos.0, pos.1 - 1))
    }
    if right {
        next.push(Pos(pos.0, pos.1 + 1))
    }
    next
}

fn print_graph(graph: &Vec<Vec<char>>, visited: &HashSet<Pos>) {
    for (r, row) in graph.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'.' {
                print!("\u{1b}[1m\u{1b}[31m{ch}\u{1b}[0m");
            } else if visited.contains(&Pos(r as i32, c as i32)) {
                print!("\u{1b}[1m\u{1b}[32m{ch}\u{1b}[0m");
            } else {
                print!("{ch}");
            }
        }
        println!();
    }
    println!();
}

fn part1(input: &str) -> usize {
    // bfs graph with rules
    // Start at S
    // A location can only connect to two locations:
    // So we should check up, down, left, right positions based on which pipe we are on.
    // 'S' is a special case since we need to check all directions to infer which pipe it is.

    // Find S
    let graph = parse(input);
    let start = find_start(&graph);
    let mut visited: HashSet<Pos> = HashSet::from([start]);
    let mut queue = get_start(&start, &graph);
    let mut steps = 0;

    while queue.len() > 0 {
        let mut next_positions = Vec::new();
        for pos in queue.iter() {
            visited.insert(*pos);
            let Some(next) = get_next(pos, &graph, &visited) else {
                continue;
            };
            // visit location
            visited.insert(next);
            next_positions.push(next);
        }
        queue = next_positions;
        steps += 1;
    }
    print_graph(&graph, &visited);
    steps
}

fn part2(input: &str) -> usize {
    // We need to figure out how many '.'s are within the loop...
    // Can we do a DFS and check which '.'s are on the "right hand" side of the loop?
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_square() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_complex() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(input), 8);
    }
}
