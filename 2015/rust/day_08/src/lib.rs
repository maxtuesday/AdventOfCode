pub fn process_part_1(input: &str) -> String {
    let x = input.lines()
        .map(|line| {
            let char_code_count = line.len() as i32;
            let chars = line.chars().collect::<Vec<_>>();
            let mut i = 1;
            let mut char_count = 0;
            while i < line.len() - 1 {
                char_count += 1;
                if chars[i] == '\\' {
                    i += 1;
                    if chars[i] == 'x' {
                        i += 2;
                    }
                }
                i += 1;
            }
            char_code_count - char_count
        })
        .sum::<i32>();

    x.to_string()
}

fn encode_string(s: &str) -> String {
    let mut encoded = String::from("");
    for c in s.chars() {
        if c == '\\' || c == '\"' {
            encoded.push('\\');
        }
        encoded.push(c);
    }
    encoded
}

pub fn process_part_2(input: &str) -> String {
    let x = input.lines()
        .map(|line| {
            let original_char_count = line.len() as i32;
            let encoded_string_count = encode_string(line).len() + 2;
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
