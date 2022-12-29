pub fn process_part_1(input: &str) -> String {
    let x = input.lines()
        .map(|line| {
            let char_code_count = line.len() as i32;
            let mut i = 0;
            let mut char_count = 0;
            let chars = line.chars().collect::<Vec<_>>();
            while i < line.len() {
                match chars[i] {
                    'a'..='z' => char_count += 1,
                    '\\' => { // escape char
                        i += 1;
                        match chars[i] {
                            '\\' | '\"' => char_count += 1,
                            'x' => {
                                char_count += 1;
                                i += 2; // skip hex code
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            char_code_count - char_count
        })
        .sum::<i32>();

    x.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let x = input.lines()
        .map(|line| {
            let original_char_count = line.len() as i32;
            let encoded_string_count = line.chars()
                .map(|char| {
                    match char {
                        '\\' | '\"' => format!("\\{}", char),
                        _ => char.to_string()
                    }
                })
                .collect::<String>()
                .len() + 2;
            encoded_string_count as i32 - original_char_count
        })
        .sum::<i32>();

    x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "12");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "19");
    }
}
