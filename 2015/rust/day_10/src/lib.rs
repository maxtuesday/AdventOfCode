fn look_and_say(input: String) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let mut cur_char = chars[0];
    let mut cur_char_count = 0;
    let mut res = String::from("");
    for c in chars {
        if c != cur_char { // new character
            res.push_str(format!("{cur_char_count}{cur_char}").as_str());
            cur_char_count = 1;
            cur_char = c;
        } else {
            cur_char_count += 1;
        }
    }
    res.push_str(format!("{cur_char_count}{cur_char}").as_str());
    res
}

fn look_and_say_times(start: String, iterations: usize) -> String {
    (0..iterations).fold(start, |acc, _| {
        look_and_say(acc)
    })
}

pub fn process_part_1(input: &str) -> String {
    let res = look_and_say_times(input.to_string(), 40);
    res.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let res = look_and_say_times(input.to_string(), 50);
    res.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say_1() {
        assert_eq!(look_and_say("1".to_string()), "11");
    }

    #[test]
    fn test_look_and_say_11() {
        assert_eq!(look_and_say("11".to_string()), "21");
    }

    #[test]
    fn test_look_and_say_21() {
        assert_eq!(look_and_say("21".to_string()), "1211");
    }

    #[test]
    fn test_look_and_say_1211() {
        assert_eq!(look_and_say("1211".to_string()), "111221");
    }

    #[test]
    fn test_look_and_say_111221() {
        assert_eq!(look_and_say("111221".to_string()), "312211");
    }
}
