use std::str::Chars;

fn main() {
    let input = include_str!("../../../input/day09.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    decompress_len(&mut chars, false)
}

fn part2(input: &str) -> usize {
    let mut chars = input.chars();
    decompress_len(&mut chars, true)
}

fn parse_marker(chars: &mut Chars) -> (usize, usize) {
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
    
    (seq_len, repeat)
}

fn decompress_len(chars: &mut Chars, full: bool) -> usize {
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let (seq_len, repeat) = parse_marker(chars);

                let seq_len_add = if full {
                    let sequence = chars.clone().take(seq_len).collect::<String>();
                    let mut seq_chars = sequence.chars();
                    decompress_len(&mut seq_chars, full)
                } else {
                    seq_len
                };

                for _ in 0..seq_len {
                    chars.next();
                }
                
                return seq_len_add * repeat + decompress_len(chars, full);
            }
            _ => {
                // single character
                return 1 + decompress_len(chars, full);
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
