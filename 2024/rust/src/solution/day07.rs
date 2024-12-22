use super::Solution;

pub struct Day07;

#[derive(Debug)]
struct Calibration {
    result: usize,
    constants: Vec<usize>,
}

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Calibration {
    fn from_line(line: &str) -> Self {
        // R: X Y Z...
        let (result, nums) = line.split_once(":").expect("should only contain one ':'");
        let result = result
            .parse::<usize>()
            .expect("tokens before ':' should be valid number");
        let constants = nums
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        Self { result, constants }
    }

    fn test(&self, operators: &Vec<Operator>) -> bool {
        self.evaluate(0, self.constants[0], operators)
    }

    fn evaluate(&self, index: usize, result: usize, operators: &Vec<Operator>) -> bool {
        let next_index = index + 1;
        if next_index >= self.constants.len() {
            return result == self.result;
        }

        if result > self.result {
            // we have no operators to reduce the value
            // so we can escape early
            return false;
        }

        let val = self.constants[next_index];
        for op in operators {
            let result = match op {
                Operator::Add => result + val,
                Operator::Mul => result * val,
                Operator::Concat => format!("{result}{val}")
                    .parse::<usize>()
                    .expect("concat produces valid number"),
            };
            if self.evaluate(next_index, result, operators) {
                return true;
            }
        }
        false
    }
}

fn parse(input: &str) -> Vec<Calibration> {
    input.lines().map(Calibration::from_line).collect()
}

fn valid_calibration_total(calibrations: Vec<Calibration>, operators: Vec<Operator>) -> usize {
    calibrations
        .iter()
        .filter(|cal| cal.test(&operators))
        .map(|cal| cal.result)
        .sum()
}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let calibrations = parse(input);
        let total = valid_calibration_total(calibrations, vec![Operator::Add, Operator::Mul]);
        format!("{total}")
    }

    fn part2(&self, input: &str) -> String {
        let calibrations = parse(input);
        let total = valid_calibration_total(
            calibrations,
            vec![Operator::Add, Operator::Mul, Operator::Concat],
        );
        format!("{total}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1_example() {
        let expected = String::from("3749");
        let actual = Day07.part1(INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example() {
        let expected = String::from("11387");
        let actual = Day07.part2(INPUT);

        assert_eq!(actual, expected);
    }
}
