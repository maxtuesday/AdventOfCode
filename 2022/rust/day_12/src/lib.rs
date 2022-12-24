use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    r: i32,
    c: i32,
}

struct HeightMap {
    map: Vec<Vec<i32>>,
    start: Point,
    end: Point,
}

impl HeightMap {
    fn get_height(&self, point: &Point) -> i32 {
        self.map[point.r as usize][point.c as usize]
    }

    // BFS
    fn find_min_path(&self, start: &Point) -> Option<i32> {
        let mut step = 0;
        let mut queue: VecDeque<Point> = VecDeque::from([start.clone()]);
        let mut visited: HashSet<Point> = HashSet::from([start.clone()]);
        let height = self.map.len() as i32;
        let width = self.map[0].len() as i32;
        let dr = vec![1, 0, -1, 0];
        let dc = vec![0, 1, 0, -1];

        while queue.len() > 0 {
            let cur_len = queue.len();
            for _ in 0..cur_len {
                let pos = queue.pop_front().unwrap();
                if pos == self.end {
                    return Some(step);
                }
                for j in 0..4 {
                    let next = Point {
                        r: pos.r + dr[j],
                        c: pos.c + dc[j],
                    };
                    if is_out_of_bounds(&next, width, height) || visited.contains(&next) {
                        continue;
                    }
                    let height_diff = self.get_height(&next) - self.get_height(&pos);
                    if height_diff <= 1 {
                        queue.push_back(next.clone());
                        visited.insert(next);
                    }
                }
            }
            step += 1;
        }
        None
    }

    fn get_lowest_heights(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for (i, row) in self.map.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                if *height == 0 {
                    points.push(Point { r: i as i32, c: j as i32 });
                }
            }
        }
        points
    }
}

fn new_height_map(input: &str) -> HeightMap {
    let mut start = Point { r: 0, c: 0 };
    let mut end = Point { r: 0, c: 0 };
    let map = input.lines().enumerate().map(|(r, line)| {
        line.chars().enumerate().map(|(c, char)| {
            match char {
                'S' => {
                    start = Point { r: r as i32, c: c as i32 };
                    0
                }
                'E' => {
                    end = Point { r: r as i32, c: c as i32 };
                    26
                }
                _ => char as i32 - 'a' as i32
            }
        }).collect()
    }).collect();
    HeightMap { map, start, end }
}

fn is_out_of_bounds(point: &Point, w: i32, h: i32) -> bool {
    point.r < 0 || point.r >= h || point.c < 0 || point.c >= w
}

pub fn process_part_1(input: &str) -> String {
    let hm = new_height_map(input);
    hm.find_min_path(&hm.start).unwrap().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let hm = new_height_map(input);
    let min_distance = hm.get_lowest_heights()
        .iter()
        .filter_map(|point| hm.find_min_path(point))
        .min()
        .unwrap();
    min_distance.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_small_input() {
        let file = fs::read_to_string("./../../inputs/12/small_input.txt").unwrap();
        assert_eq!(process_part_1(&file), "31")
    }

    #[test]
    fn part2_small_input() {
        let file = fs::read_to_string("./../../inputs/12/small_input.txt").unwrap();
        assert_eq!(process_part_2(&file), "29")
    }
}
