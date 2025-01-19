use rayon::prelude::*;

use crate::solution::Solution;

pub struct Day05;

fn check_chars(a: char, b: char) -> bool {
    let same_letter = a.to_lowercase().cmp(b.to_lowercase()).is_eq();
    let diff_case_1 = a.is_uppercase() && b.is_lowercase();
    let diff_case_2 = a.is_lowercase() && b.is_uppercase();
    same_letter && (diff_case_1 || diff_case_2)
}

// Try a two pointer strategy for faster solution:
// find a two sequencial same letters where one is UPPERCASE and other is LOWERCASE
// After finding this window, we will expand the window until we do not find this pattern.
//
// Once the pattern is finished, we will take all the characters prior to "L" pointer and add them to the reduced string.
// Then continue from "R+1" and continue the process.

fn process_simple(polymer: &[u8]) -> String {
    let mut reduced = String::new();

    let mut i = 0;
    let n = polymer.len();
    while i < n - 1 {
        if check_chars(polymer[i] as char, polymer[i + 1] as char) {
            // skip these indexes
            i += 2;
        } else {
            // add ploymer[i] to reduced
            reduced.push(polymer[i] as char);
            i += 1;
        }
    }

    if i < n {
        reduced.push(polymer[i] as char);
    }

    reduced
}

fn reduce_polymer(input: String) -> String {
    let l = input.len();
    let reduced = process_simple(input.as_bytes());

    if l == reduced.len() {
        return reduced;
    }

    reduce_polymer(reduced)
}

fn remove_letter(polymer: &str, letter: char) -> String {
    polymer
        .replace(letter.to_uppercase().to_string().as_str(), "")
        .replace(letter.to_lowercase().to_string().as_str(), "")
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let reduced = reduce_polymer(input.to_string());
        let len = reduced.len();
        format!("{len}")
    }

    fn part2(&self, input: &str) -> String {
        // Try removing each letter then reducing
        let min_len = ('a'..='z')
            .into_par_iter()
            .map(|letter| {
                let polymer = remove_letter(input, letter);
                let reduced = reduce_polymer(polymer);
                reduced.len()
            })
            .min()
            .expect("should contain at least one value");
        format!("{min_len}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = "10";
        let actual = Day05.part1(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = "4";
        let actual = Day05.part2(input);

        assert_eq!(actual, expected);
    }
}
