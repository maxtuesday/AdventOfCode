fn main() {
    let input = include_str!("../../../input/day05.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let door_id = input.as_bytes();

    let mut password = String::new();

    for i in 0..usize::MAX {
        let digest = md5::compute([door_id, i.to_string().as_bytes()].concat());
        if digest[..2] == [0, 0] && digest[2] <= 0x0f {
            password.push(char::from_digit(digest[2] as u32, 16).unwrap());
            if password.len() == 8 {
                break;
            }
        }
    }

    password
}

fn part2(input: &str) -> String {
    let door_id = input.as_bytes();

    let mut password = vec!['_'; 8];
    let mut count = 0;

    for i in 0..usize::MAX {
        let digest = md5::compute([door_id, i.to_string().as_bytes()].concat());
        if digest[..2] == [0, 0] && digest[2] <= 0x0f {
            let hex = format!("{:x}", digest);
            let pos = hex.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
            let ch = hex.chars().nth(6).unwrap();

            if pos <= 7 && password[pos] == '_' {
                password[pos] = ch;
                count += 1;
            }

            if count == 8 {
                break;
            }
        }
    }

    password.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let input = "abc";
        let expected = String::from("18f47a30");
        assert_eq!(part1(input), expected);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "abc";
        let expected = String::from("05ace8e3");
        assert_eq!(part2(input), expected);
    }
}
