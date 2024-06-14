fn main() {
    let input = include_str!("../../../input/day09.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    parse(input).len()
}

fn parse(input: &str) -> String {
    // step through characters until we get to an open paren
    let mut new_str = String::new();

    let chars = &mut input.chars();
    while let Some(char) = chars.next() {
        match char {
            '(' => {
                // take until find 'x'
                let seq_len = chars
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker length");
                
                let times = chars
                    .take_while(|c| *c != ')')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker repeat times");
                

                // take for seq_len
                let sequence = chars.take(seq_len).collect::<String>();

                new_str.push_str(&sequence.repeat(times));
            }
            c => {
                new_str.push(c);
            }
        }
    }

    new_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = "ADVENT";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part1_ex2() {
        let input = "A(1x5)BC";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part1_ex3() {
        let input = "(3x3)XYZ";
        assert_eq!(part1(input), 9);
    }

    #[test]
    fn test_part1_ex4() {
        let input = "A(2x2)BCD(2x2)EFG";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part1_ex5() {
        let input = "(6x1)(1x3)A";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part1_ex6() {
        let input = "X(8x2)(3x3)ABCY";
        assert_eq!(part1(input), 18);
    }
}
