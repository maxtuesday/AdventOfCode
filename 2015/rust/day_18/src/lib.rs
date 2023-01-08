pub fn process_part_1(input: &str) -> String {
    let lights = parse_input(input);
    let result_state = simulate_updates(lights, 100, vec![]);
    count_on_lights(&result_state).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let lights = parse_input(input);
    let w = lights[0].len();
    let h = lights.len();
    let stuck_lights = vec![
        (0, 0),
        (0, w - 1),
        (h - 1, 0),
        (h - 1, w - 1),
    ];
    let result_state = simulate_updates(lights, 100, stuck_lights);
    count_on_lights(&result_state).to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn simulate_updates(mut initial_state: Vec<Vec<char>>, steps: usize, stuck_lights: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    for stuck_light in &stuck_lights {
        initial_state[stuck_light.0][stuck_light.1] = '#';
    }
    for _ in 0..steps {
        initial_state = update_state(initial_state, &stuck_lights);
    }
    initial_state
}

fn update_state(state: Vec<Vec<char>>, stuck_lights: &Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut next_state = state.clone();
    for r in 0..next_state.len() {
        for c in 0..next_state[r].len() {
            next_state[r][c] = next_light_state(r, c, &state);
        }
    }
    for stuck_light in stuck_lights {
        next_state[stuck_light.0][stuck_light.1] = '#';
    }
    next_state
}

fn next_light_state(r: usize, c: usize, state: &Vec<Vec<char>>) -> char {
    let on_neighbor_count = count_on_neighbors(r as i32, c as i32, state);
    if state[r][c] == '#' {
        if on_neighbor_count == 2 || on_neighbor_count == 3 { '#' } else { '.' }
    } else {
        if on_neighbor_count == 3 { '#' } else { '.' }
    }
}

fn is_out_of_bounds(r: i32, c: i32, w: i32, h: i32) -> bool {
    r < 0 || r >= h || c < 0 || c >= w
}

fn count_on_neighbors(r: i32, c: i32, state: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let w = state[0].len() as i32;
    let h = state.len() as i32;
    for i in -1..=1 {
        for j in -1..=1 {
            let rr = i + r;
            let cc = j + c;
            if rr == r && cc == c {
                continue;
            }
            if is_out_of_bounds(rr, cc, w, h) {
                continue;
            }
            if state[rr as usize][cc as usize] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn count_on_lights(state: &Vec<Vec<char>>) -> usize {
    state.iter()
        .map(|row| {
            row.iter().filter(|c| *c == &'#').count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;

    #[test]
    fn part1() {
        let lights = parse_input(INPUT);
        let result_state = simulate_updates(lights, 4, vec![]);
        let count = count_on_lights(&result_state);
        assert_eq!(count, 4);
    }

    #[test]
    fn part2() {
        let lights = parse_input(INPUT);
        let w = lights[0].len();
        let h = lights.len();
        let stuck_lights = vec![
            (0, 0),
            (0, w - 1),
            (h - 1, 0),
            (h - 1, w - 1),
        ];
        let result_state = simulate_updates(lights, 5, stuck_lights);
        let count = count_on_lights(&result_state);
        assert_eq!(count, 17);
    }
}
