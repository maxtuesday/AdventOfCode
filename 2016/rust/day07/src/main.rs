fn main() {
    let input = include_str!("../../../input/day07.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq)]
struct IpAddrV7 {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

impl IpAddrV7 {
    fn from_str(s: &str) -> Self {
        // supernets are not surrounded by square brackets
        // hypernets are surrounded by square brackets []

        let mut supernets = Vec::new();
        let mut hypernets = Vec::new();

        let mut current_block = String::new();
        for c in s.chars() {
            if c == '[' {
                supernets.push(current_block.clone());
                current_block = String::new();
            } else if c == ']' {
                hypernets.push(current_block.clone());
                current_block = String::new();
            } else {
                current_block.push(c);
            }
        }

        if current_block.len() > 0 {
            supernets.push(current_block);
        }

        Self {
            supernets,
            hypernets,
        }
    }

    fn supports_tls(&self) -> bool {
        // check that there is no "abba" pattern in hypernets
        let pattern_in_hypersets = self
            .hypernets
            .iter()
            .find(|hypernet| contains_abba_pattern(&hypernet))
            .is_some();

        if pattern_in_hypersets {
            return false;
        }

        self.supernets
            .iter()
            .find(|supernet| contains_abba_pattern(&supernet))
            .is_some()
    }

    fn supports_ssl(&self) -> bool {
        // any supernet contains an "aba" pattern,
        // and that pattern exists in a hypernet as a "bab" pattern
        self.supernets
            .iter()
            .flat_map(|supernet| {
                supernet
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(3)
                    .filter(|window| window[0] == window[2] && window[0] != window[1])
                    .map(|window| format!("{b}{a}{b}", b = window[1], a = window[0]))
                    .collect::<Vec<_>>()
            })
            .find(|bab| {
                self.hypernets
                    .iter()
                    .find(|hypernet| hypernet.contains(bab))
                    .is_some()
            })
            .is_some()
    }
}

fn contains_abba_pattern(s: &String) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .find(|window| window[0] == window[3] && window[1] == window[2] && window[0] != window[1])
        .is_some()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| IpAddrV7::from_str(line).supports_tls())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| IpAddrV7::from_str(line).supports_ssl())
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
    fn test_ip_addr_v7_from_str_1() {
        let expected = IpAddrV7 {
            supernets: vec![String::from("abba"), String::from("qrst")],
            hypernets: vec![String::from("mnop")],
        };
        assert_eq!(IpAddrV7::from_str("abba[mnop]qrst"), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        let input = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
";
        assert_eq!(part2(input), 3);
    }
}
