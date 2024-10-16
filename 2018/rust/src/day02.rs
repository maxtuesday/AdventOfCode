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

fn single_char_diff(a: &str, b: &str) -> Option<usize> {
    assert!(a.len() == b.len());

    let a = a.to_string();
    let b = b.to_string();

    let mut diffs = 0;
    let mut first_diff_idx = 0;
    for i in 0..a.len() {
        if a.as_bytes()[i] != b.as_bytes()[i] {
            diffs+=1;
            if diffs == 1 {
                first_diff_idx = i;
            } else {
                return None
            }
        }
    }
    Some(first_diff_idx)
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
        let boxes = input.lines().collect::<Vec<_>>();

        for i in 0..boxes.len() {
            for j in i+1..boxes.len() {
                if let Some(idx) = single_char_diff(boxes[i], boxes[j]) {
                    let s = boxes[i];
                    let left = &s[..idx];
                    let right = &s[idx+1..];
                    return format!("{left}{right}")
                }
            }
        }
        unimplemented!()
    }
}
