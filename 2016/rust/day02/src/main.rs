fn main() {
    let input = include_str!("../../../input/day02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    // keypad:
    // 1 2 3
    // 4 5 6
    // 7 8 9

    let keypad = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let mut x = 1i32;
    let mut y = 1i32;
    let mut code = String::new();
    for line in input.lines() {
        for dir in line.chars() {
            match dir {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                c => unimplemented!("unknown direction: {c}"),
            }
            x = x.clamp(0, 2);
            y = y.clamp(0, 2);
        }
        code.push(keypad[y as usize][x as usize]);
    }
    code
}

fn part2(input: &str) -> String {
    /*
    keypad:
        1
      2 3 4
    5 6 7 8 9
      A B C
        D
    */

    let keypad = vec![
        vec!['x', 'x', '1', 'x', 'x'],
        vec!['x', '2', '3', '4', 'x'],
        vec!['5', '6', '7', '8', '9'],
        vec!['x', 'A', 'B', 'C', 'x'],
        vec!['x', 'x', 'D', 'x', 'x'],
    ];

    // start at '5'
    let mut r = 2i32;
    let mut c = 0i32;
    let mut code = String::new();
    for line in input.lines() {
        for dir in line.chars() {
            match dir {
                'U' => r -= 1,
                'D' => r += 1,
                'L' => c -= 1,
                'R' => c += 1,
                c => unimplemented!("unknown direction: {c}"),
            }
            r = r.clamp(0, 4);
            c = c.clamp(0, 4);
            if keypad[r as usize][c as usize] == 'x' {
                // undo move
                match dir {
                    'U' => r += 1,
                    'D' => r -= 1,
                    'L' => c += 1,
                    'R' => c -= 1,
                    c => unimplemented!("unknown direction: {c}"),
                }
            }
        }
        code.push(keypad[r as usize][c as usize]);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(INPUT), "1985");
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(INPUT), "5DB3");
    }
}
