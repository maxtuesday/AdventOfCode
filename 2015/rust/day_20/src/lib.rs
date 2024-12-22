// Credit: https://markheath.net/post/advent-of-code-day20

pub fn process_part_1(input: &str) -> String {
    let target = input.parse::<usize>().unwrap();
    let fact = 2 * 3 * 5 * 7 * 11;
    for house in 700000..=target / 10 {
        if house % fact == 0 {
            let presents = presents_for_house(house);
            if presents >= target {
                return house.to_string();
            }
        }
    }
    panic!("Answer not found")
}

fn presents_for_house(house: usize) -> usize {
    (1..=house)
        .filter(|elf| house % elf == 0)
        .map(|x| x * 10)
        .sum::<usize>()
}

pub fn process_part_2(input: &str) -> String {
    let target = input.parse::<usize>().unwrap();
    let fact = 2 * 2 * 2 * 3 * 3;
    for house in 776160..=target / 10 {
        if house % fact == 0 {
            let presents = presents_for_house_part_2(house);
            if presents >= target {
                return house.to_string();
            }
        }
    }
    panic!("Answer not found")
}

fn presents_for_house_part_2(house: usize) -> usize {
    (1..=house)
        .filter(|elf| house % elf == 0 && house / elf <= 50)
        .map(|x| x * 11)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "33100000";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "776160");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "786240");
    }
}
