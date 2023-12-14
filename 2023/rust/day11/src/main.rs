fn main() {
    let input = include_str!("../../../input/day11.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_empty_rows(universe: &Vec<Vec<char>>) -> Vec<usize> {
    universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect()
}

fn get_empty_cols(universe: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_cols = Vec::new();
    for c in 0..universe[0].len() {
        let mut is_empty = true;
        for r in 0..universe.len() {
            if universe[r][c] != '.' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_cols.push(c);
        }
    }
    empty_cols
}

// Do not use!
fn expand(universe: &mut Vec<Vec<char>>, scale: usize) {
    let empty_rows = get_empty_rows(universe);
    let empty_cols = get_empty_cols(universe);

    for row in empty_rows.into_iter().rev() {
        for _ in 1..scale {
            universe.insert(row, universe[row].clone());
        }
    }

    for col in empty_cols.into_iter().rev() {
        for _ in 1..scale {
            for row in 0..universe.len() {
                universe[row].insert(col, '.');
            }
        }
    }
}

#[derive(Debug)]
struct Pos {
    r: usize,
    c: usize,
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut galaxies = Vec::new();
    for r in 0..universe.len() {
        for c in 0..universe[r].len() {
            if universe[r][c] == '#' {
                galaxies.push(Pos { r, c });
            }
        }
    }
    galaxies
}

// We should not actually expand the universe
// Just keep track of the empty rows and cols and if any galaxy pair includes them,
// add more distance based on the scale
fn distance(a: &Pos, b: &Pos, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>, scale: usize) -> usize {
    let ar = a.r;
    let br = b.r;
    let dy_range = ar.min(br)..=ar.max(br);
    let empty_rows_in_range = empty_rows.iter().filter(|row| dy_range.contains(row)).count();
    
    let ac = a.c;
    let bc = b.c;
    let dx_range  = ac.min(bc)..=ac.max(bc);
    let empty_cols_in_range = empty_cols.iter().filter(|col| dx_range.contains(col)).count();

    // add rows * (scale - 1)
    let dy = a.r.abs_diff(b.r) + (empty_rows_in_range * (scale - 1));
    // add cols * (scale - 1)
    let dx = a.c.abs_diff(b.c) + (empty_cols_in_range * (scale - 1));
    dy + dx
}

fn print_universe(universe: &Vec<Vec<char>>) {
    for r in universe.iter() {
        for c in r.iter() {
            print!("{c}");
        }
        println!()
    }
    println!()
}

fn part1(input: &str) -> usize {
    // expand universe
    // -> add a row where there are no galaxies
    // -> add a col where there are no galaxies
    let universe = parse(input);
    let empty_rows = get_empty_rows(&universe);
    let empty_cols = get_empty_cols(&universe);
    // expand(&mut universe, 2);

    // Find all galaxies
    let galaxies = find_galaxies(&universe);
    
    // Find shortest paths between all pairs of galaxies
    // path distance is rows + cols
    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            distances.push(distance(&galaxies[i], &galaxies[j], &empty_rows, &empty_cols, 2));
        }
    }
    distances.iter().sum()
}

// Actual implementation should be:
// For each galaxy pair, find how many empty rows and cols there are between their paths
// Add the expand scale to their distances
fn part2(input: &str, scale: usize) -> usize {
    let universe = parse(input);
    // Find all galaxies
    let galaxies = find_galaxies(&universe);
    let empty_rows = get_empty_rows(&universe);
    let empty_cols = get_empty_cols(&universe);

    // Find shortest paths between all pairs of galaxies
    // path distance is rows + cols
    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            distances.push(distance(&galaxies[i], &galaxies[j], &empty_rows, &empty_cols, scale));
        }
    }
    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT:&str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 374);
    }

    #[test]
    fn test_expand_2x() {
        let mut input = parse(INPUT);
        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        expand(&mut input, 2);
        assert_eq!(input, parse(expected));
    }

    #[test]
    fn test_part2_10x() {
        assert_eq!(part2(INPUT, 10), 1030);
    }

    #[test]
    fn test_part2_100x() {
        assert_eq!(part2(INPUT, 100), 8410);
    }
}
