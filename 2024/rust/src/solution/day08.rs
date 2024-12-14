use std::collections::HashSet;

use super::Solution;

pub struct Day08;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pos {
    r: i32,
    c: i32,
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    pos: Pos,
}

fn parse(input: &str) -> (Vec<Antenna>, (i32, i32)) {
    let r_bounds = input.lines().count();
    let c_bounds = input
        .lines()
        .take(1)
        .map(|line| line.chars().count())
        .collect::<Vec<_>>()[0];

    let antannea = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(c, char)| match char {
                    '.' => None,
                    char => Some(Antenna {
                        frequency: char,
                        pos: Pos {
                            r: r as i32,
                            c: c as i32,
                        },
                    }),
                })
                .collect::<Vec<Antenna>>()
        })
        .collect();

    (antannea, (r_bounds as i32, c_bounds as i32))
}

fn input_as_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn display(grid: &Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|char| {
            print!("{char}");
        });
        println!()
    });
    println!()
}

#[derive(Debug)]
struct Slope {
    rise: i32,
    run: i32,
}

fn slope(a: &Antenna, b: &Antenna) -> Slope {
    // rise over run
    // (1, 8)
    // (2, 5)
    // rise: 8 - 5 = 3
    // run: 1 - 2 = -1
    let rise = a.pos.r - b.pos.r;
    let run = a.pos.c - b.pos.c;
    Slope { rise, run }
}

fn is_oob(p: &Pos, r_bounds: i32, c_bounds: i32) -> bool {
    p.r < 0 || r_bounds <= p.r || p.c < 0 || c_bounds <= p.c
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        let (antennae, (r_bounds, c_bounds)) = parse(input);

        // Group antennas by frequency
        let frequencies = antennae
            .iter()
            .map(|antenna| antenna.frequency)
            .collect::<HashSet<char>>()
            .into_iter()
            .collect::<Vec<char>>();


        let mut antinode_locations: HashSet<Pos> = HashSet::new();

        for frequency in frequencies {
            let group = antennae
                .iter()
                .filter(|antenna| antenna.frequency == frequency)
                .collect::<Vec<_>>();

            for i in 0..group.len() {
                for j in i + 1..group.len() {
                    let a = group[i];
                    let b = group[j];
                    // compare with other antannae with same frequency.
                    // Add an antinode (one for each antennae) (as long as it lands within the bounds of the map)
                    // On the opposite side of the other, same distance away as the two antennae are from each other.

                    // This is slope of a traveling to b
                    let slope = slope(a, b);

                    // check if antinode positions land within the bounds

                    // antinode on other size of b (produced by a)
                    // Need to flip the signs on the slope rise/run
                    let antinode_a_pos = Pos {
                        r: b.pos.r - slope.rise,
                        c: b.pos.c - slope.run,
                    };
                    if !is_oob(&antinode_a_pos, r_bounds, c_bounds) {
                        antinode_locations.insert(antinode_a_pos);
                    }

                    // antinode on other size of a (produced by b)
                    let antinode_b_pos = Pos {
                        r: a.pos.r + slope.rise,
                        c: a.pos.c + slope.run,
                    };
                    if !is_oob(&antinode_b_pos, r_bounds, c_bounds) {
                        antinode_locations.insert(antinode_b_pos);
                    }
                }
            }
        }

        let count = antinode_locations.len();
        format!("{count}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1_example() {
        let expected = String::from("14");
        let actual = Day08.part1(INPUT);

        assert_eq!(actual, expected);
    }
}
