fn main() {
    let input = include_str!("../../../input/day09.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|seq| process(seq.clone()))
        .sum()
}

fn process(seq: Vec<i32>) -> i32 {
    // if all nums are 0, we are done
    if seq.iter().all(|v| *v == 0) {
        return 0;
    }
    let last = seq.last().expect("should not be empty").clone();
    // find difference between all nums
    let diffs = seq
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    let n = process(diffs);
    last + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_process() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(process(input), 18);
    }

    #[test]
    fn test_process_print() {
        let input = vec![
            4, 1, -2, -5, -8, -11, -14, -17, -20, -23, -26, -29, -32, -35, -38, -41, -44, -47, -50,
            -53, -56,
        ];
        assert_eq!(process(input), -59);
    }
}
