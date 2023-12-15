fn main() {
    let input = include_str!("../../../input/day12.txt");
    println!("Part 1: {}", part1(input));
}

fn backtrack(
    s_idx: usize,
    g_idx: usize,
    springs: &Vec<char>,
    group_sizes: &Vec<usize>,
    count: &mut usize,
) {
    if s_idx >= springs.len() && g_idx >= group_sizes.len() {
        *count += 1;
        return;
    } else if s_idx >= springs.len() || g_idx >= group_sizes.len() {
        return;
    }

    let group_size = group_sizes[g_idx];
    match springs[s_idx] {
        '#' => {
            // For the given group size, can we make a group here?
            if s_idx + group_size > springs.len() {
                // we will be out of bounds
                return;
            }

            let group = &springs[s_idx..s_idx + group_size];
            let last = springs.get(s_idx + group_size);
            let is_valid_group = group.iter().all(|c| matches!(c, '#' | '?'));
            let is_valid = is_valid_group && (matches!(last.unwrap_or(&'.'), '.' | '?'));
            println!(
                "{:?} {:?} | {}",
                &springs[s_idx..s_idx + group_size],
                last,
                is_valid
            );

            if is_valid {
                // keep going
                backtrack(
                    s_idx + group_size + 1,
                    g_idx + 1,
                    springs,
                    group_sizes,
                    count,
                );
            }
            // other wise do nothing, break early...
        }
        '.' => {
            backtrack(s_idx + 1, g_idx, springs, group_sizes, count);
        }
        '?' => {
            // Skip or Take
            backtrack(s_idx + 1, g_idx, springs, group_sizes, count);

            // Take
            if s_idx + group_size >= springs.len() {
                // we will be out of bounds
                return;
            }

            let group = &springs[s_idx..s_idx + group_size];
            let last = springs.get(s_idx + group_size);
            let is_valid_group = group.iter().all(|c| matches!(c, '#' | '?'));
            let is_valid = is_valid_group && (matches!(last.unwrap_or(&'.'), '.' | '?'));
            println!(
                "{:?} {:?} | {}",
                &springs[s_idx..s_idx + group_size],
                last,
                is_valid
            );

            if is_valid {
                // keep going
                backtrack(
                    s_idx + group_size + 1,
                    g_idx + 1,
                    springs,
                    group_sizes,
                    count,
                );
            }
        }
        c => unimplemented!("unrecognized char: {c}"),
    }
}

fn compute_arrangements(springs: &Vec<char>, group_sizes: &Vec<usize>) -> usize {
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
    0
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    let springs = parts[0].chars().collect::<Vec<_>>();
    let groups = parts[1]
        .split(",")
        .filter_map(|d| d.parse::<usize>().ok())
        .collect::<Vec<_>>();
    (springs, groups)
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    input.lines().for_each(|line| {
        let (springs, group_sizes) = parse_line(line);
        backtrack(0, 0, &springs, &group_sizes, &mut count);
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_backtrack_1() {
        let input = "???.### 1,1,3";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_backtrack_2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_backtrack_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_backtrack_4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_backtrack_5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_backtrack_6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(part1(input), 10);
    }
}
