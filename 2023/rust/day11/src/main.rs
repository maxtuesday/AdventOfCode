fn main() {
    let input = include_str!("../../../input/day11.txt");
    println!("Part 1: {}", part1(input));
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
        .collect::<Vec<_>>()
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

fn expand(universe: &mut Vec<Vec<char>>) {
    let empty_rows = get_empty_rows(universe);
    let empty_cols = get_empty_cols(universe);

    for row in empty_rows.into_iter().rev() {
        universe.insert(row, universe[row].clone());
    }

    for col in empty_cols.into_iter().rev() {
        for row in 0..universe.len() {
            universe[row].insert(col, '.');
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

fn distance(a: &Pos, b: &Pos) -> usize {
    let dy = a.r.abs_diff(b.r);
    let dx = a.c.abs_diff(b.c);
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
    let mut universe = parse(input);
    expand(&mut universe);

    // Find all galaxies
    let galaxies = find_galaxies(&universe);
    
    // Find shortest paths between all pairs of galaxies
    // path distance is rows + cols
    let mut distances = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            distances.push(distance(&galaxies[i], &galaxies[j]));
        }
    }
    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_expand() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut input = parse(input);
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
        expand(&mut input);
        assert_eq!(input, parse(expected));
    }
}
