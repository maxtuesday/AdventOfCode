fn main() {
    let input = include_str!("../../../input/day03.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_valid_tri(a: u32, b: u32, c: u32) -> bool {
    return a + b > c && b + c > a && c + a > b;
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let sides = line
                .split(' ')
                .filter_map(|num| num.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();

            assert!(sides.len() == 3, "sides does not have 3 numbers");
            sides
        })
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|sides| is_valid_tri(sides[0], sides[1], sides[2]))
        .count()
}

fn part2(input: &str) -> usize {
    let sides = parse(input);

    let mut count = 0;
    for r in (0..sides.len()).step_by(3) {
        for c in 0..3 {
            if is_valid_tri(sides[r][c], sides[r + 1][c], sides[r + 2][c]) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../input/day03.txt");
        assert_eq!(part1(input), 993);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../../input/day03.txt");
        assert_eq!(part2(input), 1849);
    }
}
