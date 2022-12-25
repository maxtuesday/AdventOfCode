pub fn process_part_1(input: &str) -> String {
    let max_calories = get_calories(input)
        .into_iter()
        .max()
        .unwrap();
    max_calories.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut calories = get_calories(input);
    calories.sort_by(|a,b| b.cmp(a));
    let ans = calories.iter().take(3).sum::<u32>();
    ans.to_string()
}

fn get_calories(input: &str) -> Vec<u32> {
    input.split("\n\n")
        .map(|elf| elf.lines()
            .map(|cal| cal.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "24000");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "45000");
    }
}
