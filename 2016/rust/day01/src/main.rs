use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../input/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

enum Direction {
    North,
    South,
    East,
    West,
}

enum Turn {
    Left,
    Right,
}

fn get_next_direction(dir: Direction, turn: Turn) -> Direction {
    match turn {
        Turn::Left => match dir {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        },
        Turn::Right => match dir {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        },
    }
}

fn parse_instruction(instruction: &str) -> (Turn, u32) {
    let turn = match &instruction[0..1] {
        "L" => Turn::Left,
        "R" => Turn::Right,
        c => unimplemented!("unknown turn: {c}"),
    };
    let steps = instruction[1..]
        .parse::<u32>()
        .expect("should be a number after turn direction");
    (turn, steps)
}

fn part1(input: &str) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = Direction::North;
    for instruction in input.split(", ") {
        let (turn, steps) = parse_instruction(instruction);
        let steps = steps as i32;
        dir = get_next_direction(dir, turn);
        match dir {
            Direction::North => y += steps,
            Direction::South => y -= steps,
            Direction::East => x += steps,
            Direction::West => x -= steps,
        }
    }
    x.abs() + y.abs()
}

fn part2(input: &str) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for instruction in input.split(", ") {
        let (turn, steps) = parse_instruction(instruction);
        dir = get_next_direction(dir, turn);
        for _ in 0..steps {
            match dir {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }
            if visited.contains(&(x, y)) {
                return x.abs() + y.abs();
            }
            visited.insert((x, y));
        }
    }
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let input = "R2, L3";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part1_ex2() {
        let input = "R2, R2, R2";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_ex3() {
        let input = "R5, L5, R5, R3";
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn part2_ex1() {
        let input = "R8, R4, R4, R8";
        assert_eq!(part2(input), 4);
    }
}
