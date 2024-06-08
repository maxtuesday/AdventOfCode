use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day06.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> String {
    let mut freqs: HashMap<usize, HashMap<char, u32>> = HashMap::new();
    for line in input.lines() {
        for (pos, ch) in line.chars().enumerate() {
            freqs
                .entry(pos)
                .and_modify(|f| {
                    f.entry(ch).and_modify(|x| *x += 1).or_insert(1);
                })
                .or_insert(HashMap::from([(ch, 1)]));
        }
    }

    let cols = freqs.len();
    let mut message = String::new();
    for i in 0..cols {
        let mut char_freqs = freqs.get(&i).unwrap().iter().collect::<Vec<_>>();
        char_freqs.sort_by(|a, b| {
            a.1.cmp(b.1).reverse()
        });
        message.push(char_freqs.first().unwrap().0.clone());
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), String::from("easter"));
    }
}
