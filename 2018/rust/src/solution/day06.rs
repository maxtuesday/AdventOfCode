use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Day06;

struct Coord {
    id: usize,
    x: usize,
    y: usize,
}

fn get_distance(loc: (usize, usize), coord: &Coord) -> usize {
    loc.0.abs_diff(coord.x) + loc.1.abs_diff(coord.y)
}

fn display_grid(grid: &Vec<Vec<i32>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!(" {:2} ", grid[i][j])
        }
        println!()
    }
    println!()
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let coords = input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let (x, y) = line.split_once(", ").unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                Coord { id, x, y }
            })
            .collect::<Vec<_>>();

        // find max x and y bounds for simulating the grid
        let max_x = coords.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 2;
        let max_y = coords.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 2;

        let mut grid = vec![vec![-1; max_x]; max_y];
        for y in 0..max_y {
            for x in 0..max_x {
                // get distance for each coordinate
                let coords_and_distances = coords
                    .iter()
                    .map(|coord| {
                        let dist = get_distance((x, y), coord);
                        (coord, dist)
                    })
                    .collect::<Vec<_>>();
                let (coord, min_dist) = coords_and_distances
                    .iter()
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();
                let all_min_dist_coords = coords_and_distances
                    .iter()
                    .filter(|(_, dist)| dist == min_dist)
                    .collect::<Vec<_>>();

                // only assign grid location given there is only one coordinate closest
                if all_min_dist_coords.len() == 1 {
                    grid[y][x] = coord.id as i32;
                }
            }
        }

        // display_grid(&grid);

        // keep track of the areas for each coordinate
        // check if the coordinate touches the edge of the grid, if so we will want to ignore this area
        let mut areas: HashMap<usize, usize> = HashMap::new();
        let mut infinite_areas: HashSet<usize> = HashSet::new();
        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, val)| {
                if let Ok(val) = (*val).try_into() {
                    if y == 0 || y == grid.len() - 1 || x == 0 || x == grid[0].len() - 1 {
                        infinite_areas.insert(val);
                    }
                    areas
                        .entry(val)
                        .and_modify(|x| {
                            *x += 1;
                        })
                        .or_insert(1);
                }
            });
        });

        // println!("{:?}", areas);

        let max_finite_area = areas
            .iter()
            .filter_map(|(id, area)| {
                if infinite_areas.contains(id) {
                    None
                } else {
                    Some(area)
                }
            })
            .max()
            .unwrap();

        format!("{max_finite_area}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let expected = "17";
        let actual = Day06.part1(input);

        assert_eq!(actual, expected);
    }
}
