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
    fn from(input: &str) -> Self {
        let mut start = Point { r: 0, c: 0 };
        let mut end = Point { r: 0, c: 0 };
        let map: Vec<Vec<i32>> = input.lines().enumerate().map(|(r, line)| {
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
        Self { map, start, end }
    }

    fn get_height(&self, point: &Point) -> Option<i32> {
        self.map.get(point.r as usize)
            .and_then(|row| row.get(point.c as usize))
            .and_then(|&height| Some(height))
    }

    fn find_min_path_to_height(&self, start: &Point, end_height: i32, calc_height_diff: fn(from: i32, to: i32) -> i32) -> Option<i32> {
        let mut step = 0;
        let mut queue: VecDeque<Point> = VecDeque::from([start.clone()]);
        let mut visited: HashSet<Point> = HashSet::from([start.clone()]);

        while queue.len() > 0 {
            let cur_queue_len = queue.len();
            for _ in 0..cur_queue_len {
                let pos = queue.pop_front().unwrap();
                let pos_height = self.get_height(&pos).unwrap();
                if pos_height == end_height {
                    return Some(step);
                }
                let neighbors = vec![
                    Point { r: pos.r + 1, c: pos.c },
                    Point { r: pos.r - 1, c: pos.c },
                    Point { r: pos.r, c: pos.c + 1 },
                    Point { r: pos.r, c: pos.c - 1 },
                ];
                neighbors.iter().for_each(|neighbor| {
                    if let Some(height) = self.get_height(neighbor) { // checks if point is out-of-bounds
                        if !visited.contains(neighbor) && calc_height_diff(pos_height, height) <= 1 {
                            queue.push_back(neighbor.clone());
                            visited.insert(neighbor.clone());
                        }
                    }
                });
            }
            step += 1
        }
        None
    }
}

fn climb(from: i32, to: i32) -> i32 {
    to - from
}

fn descend(from: i32, to: i32) -> i32 {
    from - to
}

pub fn process_part_1(input: &str) -> String {
    let hm = HeightMap::from(input);
    let target_height = hm.get_height(&hm.end).unwrap();
    let min_distance = hm
        .find_min_path_to_height(&hm.start, target_height, climb)
        .unwrap();
    min_distance.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let hm = HeightMap::from(input);
    let target_height = 0;
    let min_distance = hm
        .find_min_path_to_height(&hm.end, target_height, descend)
        .unwrap();
    min_distance.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_small_input() {
        let file = fs::read_to_string("./../../input/12/small_input.txt").unwrap();
        assert_eq!(process_part_1(&file), "31")
    }

    #[test]
    fn part2_small_input() {
        let file = fs::read_to_string("./../../input/12/small_input.txt").unwrap();
        assert_eq!(process_part_2(&file), "29")
    }
}
