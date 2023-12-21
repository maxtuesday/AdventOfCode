fn main() {
    let input = include_str!("../../../input/day15.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    input.split(",").map(hash).sum()
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
