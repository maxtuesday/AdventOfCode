fn main() {
    let input = include_str!("../../../input/day12.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Onsen {
    springs: Vec<char>,
    groups: Vec<usize>,
}

fn backtrack(
    s_idx: usize,
    g_idx: usize,
    cur_arr: String,
    onsen: &Onsen,
    arrangements: &mut Vec<String>,
) {
    // base case(s)
    // - We have exhausted info.springs locations
    // - We have exhausted info.groups

    if g_idx == onsen.groups.len() {
        // need to check if there are any remaining '#' characters
        if s_idx < onsen.springs.len() && onsen.springs[s_idx..].iter().any(|c| matches!(c, '#')) {
            // do not add this arrangement
            return;
        }

        // If we have exhausted all of our groups, then this should be a valid arrangement
        let cur_arr = if cur_arr.len() < onsen.springs.len() {
            cur_arr + ".".repeat(onsen.springs.len() - s_idx).as_str()
        } else {
            cur_arr
        };
        let cur_arr = String::from(&cur_arr[..onsen.springs.len()]);
        println!("{:?}", cur_arr);
        arrangements.push(cur_arr);
        // Add 1 to arrangement counter
        return;
    }

    if s_idx >= onsen.springs.len() {
        return;
    }

    // We still have arrangements to handle
    match onsen.springs[s_idx] {
        '#' | '?' => {
            // We can handle a '?' as either a '.' or '#'
            if onsen.springs[s_idx] == '?' {
                backtrack(s_idx + 1, g_idx, cur_arr.clone() + ".", onsen, arrangements);
            }

            // Handle '#'
            // Take a slice from s_idx..s_idx+info.group[g_idx]
            let group_size = onsen.groups[g_idx];
            let group_end_idx = s_idx + group_size;
            if group_end_idx > onsen.springs.len() {
                // println!("group too large for remaining springs");
                return;
            }
            let group = &onsen.springs[s_idx..group_end_idx];

            // Get the Option<char> before and after the slice
            // group validity checks
            // - before cannot be '#' since we would have handled that case earlier.
            // - after cannot be '#' since that would mean this group is touching another.
            let before = if s_idx > 0 {
                onsen.springs.get(s_idx - 1).unwrap_or(&'.')
            } else {
                &'.'
            };
            let after = onsen.springs.get(group_end_idx).unwrap_or(&'.');

            if *before == '#' || *after == '#' {
                // println!("before or after == #");
                return;
            }

            if !group.iter().all(|c| matches!(c, '#' | '?')) {
                // println!("not all chars in slice are # or ?");
                return;
            }

            backtrack(
                group_end_idx + 1,
                g_idx + 1,
                cur_arr.clone() + "#".repeat(group_size).as_str() + ".",
                onsen,
                arrangements,
            )
        }
        '.' => backtrack(s_idx + 1, g_idx, cur_arr.clone() + ".", onsen, arrangements),
        c => unimplemented!("unrecognized char: {c}"),
    }
}

fn compute_arrangements(onsen: &Onsen) -> usize {
    // Process:
    // We need to find the number of arragements that the springs could be in.
    //
    // We will step through the springs
    // Start with a group size, and see how we can fit that into the spring mapping.
    //
    // Given a '#', we will need to start the group and make sure to end the group.
    // Given a '.', there will be no spring.
    // Given a '?', We can do one of two things:
    //  1. Skip the location
    //  2. Include the location
    //     We need to check what the group size is and take a slice of that size.
    //     Given that slice, check if we can fit the group.
    //     If we can, we can continue.

    // We can refactor this to just keep track of a count
    // This vec was for debugging
    let mut arrangements = Vec::new();
    backtrack(0, 0, String::from(""), onsen, &mut arrangements);
    arrangements.len()
}

fn parse_line(line: &str) -> Onsen {
    let parts = line.split(" ").collect::<Vec<_>>();
    let springs = parts[0].chars().collect::<Vec<_>>();
    let groups = parts[1]
        .split(",")
        .filter_map(|d| d.parse::<usize>().ok())
        .collect::<Vec<_>>();
    Onsen { springs, groups }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // println!("{line}");
            let info = parse_line(line);
            compute_arrangements(&info)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let info = parse_line(line);
            let mut expanded_springs = Vec::new();
            let mut expanded_groups = Vec::new();
            for _ in 0..5 {
                expanded_springs.extend(info.springs.iter());
                expanded_springs.push('?');

                expanded_groups.extend(info.groups.iter());
            }
            expanded_springs.pop(); // remove last extra '?'

            println!("{:?}", expanded_springs);
            println!("{:?}", expanded_groups);
            let count = compute_arrangements(&Onsen {
                springs: expanded_springs,
                groups: expanded_groups,
            });
            println!("Count: {count}");
            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        let input = include_str!("../../../input/day12.txt");
        assert_eq!(part1(input), 7694);
    }

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part1_ex1() {
        let input = "???.### 1,1,3";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part1_ex2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_ex3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part1_ex4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part1_ex5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_ex6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn test_part1_input_ex1() {
        let input = "#?????#??. 1,3,2";
        // What is the correct number of arrangements?
        // #?????#??. 1,3,2
        // #.###.##..
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_part1_input_ex2() {
        let input = "?#????#?.??#?????? 3,1,1,1,1,1";
        // What is the correct number of arrangements?
        // ?#????#?.??#?????? 3,1,1,1,1,1
        // .###..#..#.#.#.#..
        // .###..#..#.#.#..#.
        // .###..#..#.#.#...#
        // .###..#..#.#..#..#
        // .###..#..#.#..#.#.
        // .###..#..#.#...#.#
        // .###..#....#.#.#.#
        //
        // ###...#....#.#.#.#
        // ###...#..#.#...#.#
        // ###...#..#.#..#..#
        // ###...#..#.#..#.#.
        // ###...#..#.#.#...#
        // ###...#..#.#.#..#.
        // ###...#..#.#.#.#..
        //
        // ###.#.#....#...#.#
        // ###.#.#....#..#..#
        // ###.#.#....#..#.#.
        // ###.#.#....#.#...#
        // ###.#.#....#.#..#.
        // ###.#.#....#.#.#..
        //
        // ###.#.#..#.#.....#
        // ###.#.#..#.#....#.
        // ###.#.#..#.#...#..
        // ###.#.#..#.#..#...
        // ###.#.#..#.#.#....
        //
        assert_eq!(part1(input), 25);
    }

    #[test]
    fn test_part1_input_ex3() {
        let input = ".??#?.?#??#??. 1,2,2";
        // .??#?.?#??#??. 1,2,2
        // ...#..##.##...
        // ...#..##..##..
        // ...#...##.##..
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2_ex() {
        assert_eq!(part2(INPUT), 525152);
    }
}
