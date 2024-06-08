use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day06.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

type FreqMap = HashMap<usize, HashMap<char, u32>>;

fn get_char_freqs(input: &str) -> FreqMap {
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
    freqs
}

fn get_message(
    freqs: FreqMap,
    order_by: fn(&(&char, &u32), &(&char, &u32)) -> std::cmp::Ordering,
) -> String {
    let mut message = String::new();
    for i in 0..freqs.len() {
        let mut char_freqs = freqs.get(&i).unwrap().iter().collect::<Vec<_>>();
        char_freqs.sort_by(order_by);
        message.push(char_freqs.first().unwrap().0.clone());
    }
    message
}

fn part1(input: &str) -> String {
    let freqs = get_char_freqs(input);
    get_message(freqs, |a, b| a.1.cmp(b.1).reverse())
}

fn part2(input: &str) -> String {
    let freqs = get_char_freqs(input);
    get_message(freqs, |a, b| a.1.cmp(b.1))
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), String::from("advent"));
    }
}
