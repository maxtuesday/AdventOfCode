use std::collections::HashSet;

fn handle_direction(dir: char, pos: (i32, i32)) -> (i32, i32) {
    let mut x = pos.0;
    let mut y = pos.1;
    match dir {
        '<' => x -= 1,
        '>' => x += 1,
        '^' => y += 1,
        'v' => y -= 1,
        _ => {}
    }
    return (x, y)
}

pub fn process_part_1(input: &str) -> String {
    let mut pos = (0,0);
    let mut houses: HashSet<(i32, i32)> = HashSet::from([pos]);
    for dir in input.chars() {
        let next = handle_direction(dir, pos);
        houses.insert(next);
        pos = next;
    }
    houses.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut santa = (0,0);
    let mut robot = (0,0);
    let mut houses: HashSet<(i32, i32)> = HashSet::from([(0,0)]);
    for (idx, dir) in input.chars().enumerate() {
        if idx % 2 == 0 {
            let next = handle_direction(dir, robot);
            houses.insert(next);
            robot = next;
        } else {
            let next = handle_direction(dir, santa);
            houses.insert(next);
            santa = next;
        }
    }
    houses.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1(">"), "2")
    }

    #[test]
    fn part1_test2() {
        assert_eq!(process_part_1("^>v<"), "4")
    }

    #[test]
    fn part1_test3() {
        assert_eq!(process_part_1("^v^v^v^v^v"), "2")
    }

    #[test]
    fn part2_test1() {
        assert_eq!(process_part_2("^>"), "3")
    }

    #[test]
    fn part2_test2() {
        assert_eq!(process_part_2("^>v<"), "3")
    }

    #[test]
    fn part2_test3() {
        assert_eq!(process_part_2("^v^v^v^v^v"), "11")
    }
}
