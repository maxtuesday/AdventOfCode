use std::str::Chars;

fn main() {
    let input = include_str!("../../../input/day09.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    decompress(&mut chars)
}

fn part2(input: &str) -> usize {
    let mut chars = input.chars();
    decompress_v2(&mut chars)
}

fn decompress(chars: &mut Chars) -> usize {
    while let Some(char) = chars.next() {
        match char {
            '(' => {
                // take until find 'x'
                let seq_len = chars
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker length");

                let repeat = chars
                    .take_while(|c| *c != ')')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker repeat times");

                // take for seq_len
                for _ in 0..seq_len {
                    chars.next();
                }
                return seq_len * repeat + decompress(chars)
            }
            _ => {
                // len += 1;
                return 1 + decompress(chars)
            }
        }
    }
    0
}

fn decompress_v2(chars: &mut Chars) -> usize {
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                // take until find 'x'
                let seq_len = chars
                    .take_while(|c| *c != 'x')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker length");

                let repeat = chars
                    .take_while(|c| *c != ')')
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("tried to parse marker repeat times");

                let sequence = chars.clone().take(seq_len).collect::<String>();
                for _ in 0..seq_len {
                    chars.next();
                }
                let mut seq_chars = sequence.chars();
                let sequence_len = decompress_v2(&mut seq_chars);
                return sequence_len * repeat + decompress_v2(chars);
            }
            _ => {
                // single character
                return 1 + decompress_v2(chars);
            }
        }
    }
    0
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

    #[test]
    fn test_part2_ex1() {
        let input = "(3x3)XYZ";
        assert_eq!(part2(input), 9);
    }

    #[test]
    fn test_part2_ex2() {
        let input = "X(8x2)(3x3)ABCY";
        assert_eq!(part2(input), 20);
    }

    #[test]
    fn test_part2_ex3() {
        let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        assert_eq!(part2(input), 241920);
    }

    #[test]
    fn test_part2_ex4() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        assert_eq!(part2(input), 445);
    }
}
