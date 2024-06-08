fn main() {
    let input = include_str!("../../../input/day05.txt");
    println!("Part 1: {}", part1(input));
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
}
