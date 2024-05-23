fn main() {
    let input = include_str!("../../../input/day03.txt");
    println!("Part 1: {}", part1(input));
}

fn is_valid_tri(a: u32, b: u32, c: u32) -> bool {
    return a + b > c && b + c > a && c + a > b;
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let sides = line
                .split(' ')
                .filter_map(|num| num.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>();
            
            assert!(sides.len() == 3, "sides does not have 3 numbers");

            is_valid_tri(sides[0], sides[1], sides[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "  10 25 5";
        assert_eq!(part1(input), 0);
    }
}
