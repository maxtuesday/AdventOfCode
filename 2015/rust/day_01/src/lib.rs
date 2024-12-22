pub fn process_part_1(input: &str) -> String {
    let mut floor = 0;
    for c in input.split("") {
        match c {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => {}
        }
    }
    floor.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut floor = 0;
    let steps = input.split("");
    for (idx, c) in steps.enumerate() {
        match c {
            "(" => floor += 1,
            ")" => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            return idx.to_string()
        }
    }
    "does not reach basement".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1("(())"), "0");
    }

    #[test]
    fn part1_test2() {
        assert_eq!(process_part_1("()()"), "0");
    }

    #[test]
    fn part1_test3() {
        assert_eq!(process_part_1("((("), "3");
    }

    #[test]
    fn part1_test4() {
        assert_eq!(process_part_1("(()(()("), "3");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(process_part_2(")"), "1");
    }

    #[test]
    fn part2_test2() {
        assert_eq!(process_part_2("()())"), "5");
    }
}
