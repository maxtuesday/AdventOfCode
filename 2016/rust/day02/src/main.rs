fn main() {
    let input = include_str!("../../../input/day02.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> String {
    // keypad:
    // 1 2 3
    // 4 5 6
    // 7 8 9

    let mut x = 0i32;
    let mut y = 0i32;
    let mut code = String::new();
    for line in input.lines() {
        for dir in line.chars() {
            match dir {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                c => unimplemented!("unknown direction: {c}"),
            }
            x = x.clamp(-1, 1);
            y = y.clamp(-1, 1);
        }
        let num = match (x, y) {
            (-1, 1) => 1,
            (0, 1) => 2,
            (1, 1) => 3,
            (-1, 0) => 4,
            (0, 0) => 5,
            (1, 0) => 6,
            (-1, -1) => 7,
            (0, -1) => 8,
            (1, -1) => 9,
            pair => unimplemented!("unknown pair: {:?}", pair),
        };
        code.push_str(format!("{}", num).as_str());
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let input = "ULL
RRDDD
LURDL
UUUUD";
        assert_eq!(part1(input), "1985");
    }
}
