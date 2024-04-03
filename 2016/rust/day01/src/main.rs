fn main() {
    let input = include_str!("../../../input/day01.txt");
    println!("Part 1: {}", part1(input));
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn part1(input: &str) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = Direction::North;
    for instruction in input.split(", ") {
        let steps = &instruction[1..]
            .parse::<i32>()
            .expect("should be a number after turn direction");

        dir = match &instruction[0..1] {
            "L" => match dir {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            "R" => match dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            c => unimplemented!("unknown turn: {c}"),
        };
        match dir {
            Direction::North => y += steps,
            Direction::South => y -= steps,
            Direction::East => x += steps,
            Direction::West => x -= steps,
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
}
