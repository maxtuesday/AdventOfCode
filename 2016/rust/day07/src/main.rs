fn main() {
    let input = include_str!("../../../input/day07.txt");
    println!("Part 1: {}", part1(input));
}

fn supports_tls(s: &str) -> bool {
    let mut in_hypernet = false;
    let mut found_abba = false;

    let chars = &s.chars().collect::<Vec<_>>()[..];
    for window in chars.windows(4) {
        if window[0] == '[' {
            in_hypernet = true;
        }

        if window[0] == ']' {
            in_hypernet = false;
            continue;
        }

        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            if in_hypernet {
                return false;
            }

            found_abba = true;
        }
    }
    found_abba
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| supports_tls(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }
}
