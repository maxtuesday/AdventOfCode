pub fn process_part_1(input: &str) -> String {
    let res = input.lines()
        .map(|line| -> u32 {
            let mut dimensions: Vec<_> = line.split("x")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            dimensions.sort();
            let mut area: u32 = 0;
            let d_len = dimensions.len();
            for i in 0..d_len {
                area += dimensions[i] * dimensions[(i+1) % d_len];
            }
            area *= 2;
            let extra = dimensions.iter().take(2).product::<u32>();
            area + extra
        })
        .sum::<u32>();
    res.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let res = input.lines()
        .map(|line| -> u32 {
            let mut dimensions: Vec<_> = line.split("x")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            dimensions.sort();
            let wrap = dimensions.iter().take(2).sum::<u32>() * 2;
            let ribbon = dimensions.iter().product::<u32>();
            wrap + ribbon
        })
        .sum::<u32>();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1("2x3x4"), "58");
    }

    #[test]
    fn part1_test2() {
        assert_eq!(process_part_1("1x1x10"), "43");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(process_part_2("2x3x4"), "34");
    }

    #[test]
    fn part2_test2() {
        assert_eq!(process_part_2("1x1x10"), "14");
    }
}
