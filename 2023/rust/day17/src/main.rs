<<<<<<< Updated upstream
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = include_str!("../../../input/day17.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 1: {}", part2(input));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
=======
fn main() {
    let input = include_str!("../../../input/day17.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    0
}

#[derive(Clone)]
>>>>>>> Stashed changes
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

<<<<<<< Updated upstream
fn next_directions(direction: &Direction) -> Vec<Direction> {
    match direction {
        Direction::Up | Direction::Down => {
            vec![direction.clone(), Direction::Left, Direction::Right]
        }
        Direction::Left | Direction::Right => {
            vec![direction.clone(), Direction::Up, Direction::Down]
        }
    }
}

type Grid = Vec<Vec<u32>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Step {
    pos: Pos,
    direction: Direction,
    steps: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    step: Step,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    grid: Grid,
}

impl Graph {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        Self { grid }
    }

    fn next_pos(&self, pos: &Pos, direction: &Direction) -> Option<Pos> {
        let (x, y) = match direction {
            Direction::Up => {
                let Some(y) = pos.y.checked_sub(1) else {
                    return None;
                };
                (pos.x, y)
            }
            Direction::Down => {
                let y = pos.y + 1;
                if y >= self.grid.len() {
                    return None;
                }
                (pos.x, y)
            }
            Direction::Left => {
                let Some(x) = pos.x.checked_sub(1) else {
                    return None;
                };
                (x, pos.y)
            }
            Direction::Right => {
                let x = pos.x + 1;
                if x >= self.grid[0].len() {
                    return None;
                }
                (x, pos.y)
            }
        };
        Some(Pos { x, y })
    }

    fn next_step(
        &self,
        state: &State,
        direction: Direction,
        min_steps: u8,
        max_steps: u8,
    ) -> Option<Step> {
        let Some(pos) = self.next_pos(&state.step.pos, &direction) else {
            return None;
        };
        let steps = if state.step.direction != direction {
            1
        } else {
            state.step.steps + 1
        };
        if state.step.steps < min_steps && state.step.direction != direction {
            return None;
        }
        let next = Step {
            pos,
            direction,
            steps,
        };
        if next.steps > max_steps {
            return None;
        }
        Some(next)
    }

    fn successors(&self, state: &State, min_steps: u8, max_steps: u8) -> Vec<Step> {
        next_directions(&state.step.direction)
            .into_iter()
            .filter_map(|dir| self.next_step(&state, dir, min_steps, max_steps))
            .collect()
    }
}

fn get_min_heatloss(graph: &Graph, min_steps: u8, max_steps: u8) -> Option<u32> {
    let start_right = State {
        step: Step {
            pos: Pos { x: 1, y: 0 },
            direction: Direction::Right,
            steps: 1,
        },
        cost: graph.grid[0][1],
    };
    let start_down = State {
        step: Step {
            pos: Pos { x: 0, y: 1 },
            direction: Direction::Down,
            steps: 1,
        },
        cost: graph.grid[1][0],
    };

    let mut visited: HashSet<Step> =
        HashSet::from([start_right.step.clone(), start_down.step.clone()]);

    let mut heap = BinaryHeap::new();
    heap.push(start_right);
    heap.push(start_down);

    let goal = Pos {
        x: graph.grid[0].len() - 1,
        y: graph.grid.len() - 1,
    };

    while let Some(state) = heap.pop() {
        if state.step.pos == goal {
            if state.step.steps < min_steps {
                continue
            }
            return Some(state.cost);
        }

        for next_step in graph.successors(&state, min_steps, max_steps) {
            let cost = graph.grid[next_step.pos.y][next_step.pos.x] + state.cost;

            if !visited.contains(&next_step) {
                visited.insert(next_step.clone());
                heap.push(State {
                    step: next_step,
                    cost,
                });
            }
        }
    }
    None
}

fn part1(input: &str) -> u32 {
    let graph = Graph::from(input);
    get_min_heatloss(&graph, 1, 3).expect("goal was found")
}

fn part2(input: &str) -> u32 {
    let graph = Graph::from(input);
    get_min_heatloss(&graph, 4, 10).expect("goal was found")
=======
#[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
    direction: Direction,
    steps_until_turn: u32,
}

impl Pos {
    fn next(&self) -> Pos {
        let mut p = self.clone();
        match self.direction {
            Direction::Up => p.y -= 1,
            Direction::Down => p.y += 1,
            Direction::Left => p.x -= 1,
            Direction::Right => p.x += 1,
        }
        p.steps_until_turn -= 1;
        p
    }

    fn is_in_bounds(&self, graph: &Graph) -> bool {
        0 <= self.y && self.y < graph.len() as i32 && 0 <= self.x && self.x < graph[0].len() as i32
    }

    fn successors(&self, graph: &Graph) -> Vec<(Pos, u32)> {
        // How should we find the successors of the current position.
        // Rule: we must turn after moving 3 spaces in the same direction

        // Can we keep track of the direction this position was traveling and how many steps it has gone in that direction?

        if self.steps_until_turn > 0 {
            // we have three choices.
            // - continue in the same direction,
            // - turn 90 degrees
            let mut turns = match self.direction {
                Direction::Up | Direction::Down => {
                    vec![
                        Pos {
                            x: self.x - 1,
                            y: self.y,
                            direction: Direction::Left,
                            steps_until_turn: self.steps_until_turn - 1,
                        },
                        Pos {
                            x: self.x + 1,
                            y: self.y,
                            direction: Direction::Right,
                            steps_until_turn: self.steps_until_turn - 1,
                        },
                    ]
                }
                Direction::Left | Direction::Right => {
                    vec![
                        Pos {
                            x: self.x,
                            y: self.y - 1,
                            direction: Direction::Up,
                            steps_until_turn: self.steps_until_turn - 1,
                        },
                        Pos {
                            x: self.x,
                            y: self.y + 1,
                            direction: Direction::Down,
                            steps_until_turn: self.steps_until_turn - 1,
                        },
                    ]
                }
            };
            let same_direction = self.next();
            turns.push(same_direction);

            // filter out positions that may be oob
            let next = turns
                .iter()
                .filter(|pos| pos.is_in_bounds(graph))
                .map(|pos| (pos, graph[pos.y as usize][pos.x as usize]))
                .collect::<Vec<_>>();
        }
        todo!()
    }
}

type Graph = Vec<Vec<i32>>;

fn dijkstra(start: Pos, goal: Pos, graph: &Graph) -> u32 {
    0
>>>>>>> Stashed changes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 102);
    }
<<<<<<< Updated upstream

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2(INPUT), 94);
    }

    #[test]
    fn test_part2_ex2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part2(input), 71);
    }
=======
>>>>>>> Stashed changes
}
