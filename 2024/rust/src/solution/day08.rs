use std::collections::HashSet;

use super::Solution;

pub struct Day08;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    r: i32,
    c: i32,
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    pos: Pos,
}

#[derive(Debug)]
struct Slope {
    rise: i32,
    run: i32,
}

impl Slope {
    fn from(a: &Antenna, b: &Antenna) -> Self {
        let rise = a.pos.r - b.pos.r;
        let run = a.pos.c - b.pos.c;
        Self { rise, run }
    }
}

struct Map {
    map: Vec<Vec<char>>,
    antennas: Vec<Antenna>,
}

impl Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let antennas = map
            .iter()
            .enumerate()
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(c, char)| match char {
                        '.' => None,
                        char => Some(Antenna {
                            frequency: char.clone(),
                            pos: Pos {
                                r: r as i32,
                                c: c as i32,
                            },
                        }),
                    })
                    .collect::<Vec<Antenna>>()
            })
            .collect();

        Self { map, antennas }
    }

    fn is_oob(&self, p: &Pos) -> bool {
        p.r < 0 || self.map.len() <= p.r as usize || p.c < 0 || self.map[0].len() <= p.c as usize
    }

    fn frequencies(&self) -> Vec<char> {
        self.antennas
            .iter()
            .map(|antenna| antenna.frequency)
            .collect::<HashSet<char>>()
            .into_iter()
            .collect::<Vec<char>>()
    }

    fn create_antinodes(&self, harmonics: bool) -> HashSet<Pos> {
        let mut antinode_locations: HashSet<Pos> = HashSet::new();

        // Group antennas by frequency
        let frequencies = self.frequencies();

        for frequency in frequencies {
            let group = self
                .antennas
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
                    let slope = Slope::from(a, b);

                    // check if antinode positions land within the bounds

                    // antinode on other size of b (produced by a)
                    // Need to flip the signs on the slope rise/run
                    let mut antinode_a_pos = Pos {
                        r: b.pos.r - slope.rise,
                        c: b.pos.c - slope.run,
                    };
                    while !self.is_oob(&antinode_a_pos) {
                        antinode_locations.insert(antinode_a_pos.clone());
                        antinode_a_pos.r -= slope.rise;
                        antinode_a_pos.c -= slope.run;
                        if !harmonics {
                            break;
                        }
                    }

                    // antinode on other size of a (produced by b)
                    let mut antinode_b_pos = Pos {
                        r: a.pos.r + slope.rise,
                        c: a.pos.c + slope.run,
                    };
                    while !self.is_oob(&antinode_b_pos) {
                        antinode_locations.insert(antinode_b_pos.clone());
                        antinode_b_pos.r += slope.rise;
                        antinode_b_pos.c += slope.run;
                        if !harmonics {
                            break;
                        }
                    }

                    if harmonics {
                        // add antennas a and b as they should also be considered antinodes
                        antinode_locations.insert(a.pos.clone());
                        antinode_locations.insert(b.pos.clone());
                    }
                }
            }
        }
        antinode_locations
    }
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        let map = Map::from(input);
        let antinodes = map.create_antinodes(false);
        let count = antinodes.len();
        format!("{count}")
    }

    fn part2(&self, input: &str) -> String {
        let map = Map::from(input);
        let antinodes = map.create_antinodes(true);
        let count = antinodes.len();
        format!("{count}")
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

    #[test]
    fn test_part2_example() {
        let expected = String::from("34");
        let actual = Day08.part2(INPUT);

        assert_eq!(actual, expected);
    }
}
