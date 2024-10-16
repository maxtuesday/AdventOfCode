use std::collections::HashMap;

use crate::solution::Solution;

pub struct Day02;

struct CandidateBox {
    two_count: usize,
    three_count: usize,
}

impl CandidateBox {
    fn from(word: &str) -> Self {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ch in word.chars() {
            counts.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut two_count = 0;
        let mut three_count = 0;
        for (_, v) in counts {
            if v == 2 {
                two_count += 1;
            } else if v == 3 {
                three_count += 1;
            }
        }

        CandidateBox {
            two_count,
            three_count,
        }
    }
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let boxes = input.lines().map(CandidateBox::from).collect::<Vec<_>>();
        let a = boxes.iter().filter(|b|b.two_count >= 1).count();
        let b = boxes.iter().filter(|b|b.three_count >= 1).count();
        let prod = a * b;
        format!("{prod}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}
