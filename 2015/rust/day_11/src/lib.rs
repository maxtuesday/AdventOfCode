use std::str::from_utf8;

pub fn process_part_1(input: &str) -> String {
    next_valid_password(input.to_string())
}

pub fn process_part_2(input: &str) -> String {
    let old_password = next_valid_password(input.to_string());
    next_valid_password(old_password)
}

fn next_valid_password(password: String) -> String {
    /*
    1.  Passwords must include one increasing straight of at least three letters,
        like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    2.  Passwords may not contain the letters i, o, or l,
        as these letters can be mistaken for other characters and are therefore confusing.
    3.  Passwords must contain at least two different, non-overlapping pairs of letters,
        like aa, bb, or zz.
     */

    let len = password.len();
    let mut new_password = increment_chars(password);

    loop {
        // (2): check if password contains a "bad" character
        // this helps to skip a lot of useless iterations
        if let Some(bad_char_idx) = find_bad_char(&new_password) {
            // we should skip all passwords that contain this character
            // we have the earliest character index,
            // so we can increment this character and set all chars after to 'a'
            let (left, _) = new_password.split_at(bad_char_idx);
            // will not need to carry due to bad character list does not contain 'z'
            let next_char = new_password.as_bytes()[bad_char_idx] + 1;
            new_password = left.to_string();
            new_password.push(next_char as char);
            let right = "a".repeat(len - bad_char_idx - 1);
            new_password.push_str(right.as_str());
        }

        // Conditions (1) and (3)
        if contains_increasing_straight(&new_password) &&
            contains_non_overlapping_pairs(&new_password) {
            return new_password;
        } else {
            new_password = increment_chars(new_password);
        }
    }
}

fn find_bad_char(s: &String) -> Option<usize> {
    s.find(|char| matches!(char, 'i' | 'o' | 'l'))
}

fn contains_increasing_straight(s: &String) -> bool {
    let increasing_straight_count = s.as_bytes()
        .windows(3)
        .filter(|window| {
            window[0] + 1 == window[1] && window[1] + 1 == window[2]
        })
        .count();
    increasing_straight_count > 0
}

fn contains_non_overlapping_pairs(s: &String) -> bool {
    let non_overlapping_pair_count = s.as_bytes()
        .windows(2)
        .enumerate()
        .filter(|(_, window)| window[0] == window[1])
        .filter(|(start_idx, window)| {
            let pair = from_utf8(window).unwrap();
            let other_pair_start_idx = s.rfind(pair).unwrap();
            start_idx + 1 != other_pair_start_idx
        })
        .count();
    non_overlapping_pair_count >= 2
}

fn increment_chars(s: String) -> String {
    let mut chars = s.bytes().rev().collect::<Vec<_>>();

    let mut carry = true;
    let mut idx = 0;
    let len = chars.len();
    while carry {
        carry = false;

        chars[idx] += 1;
        if chars[idx] > b'z' {
            chars[idx] = b'a';
            carry = true;
        }

        idx = (idx + 1) % len;
    }

    chars.reverse();
    from_utf8(chars.as_slice()).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_password_checks {
        use super::*;

        #[test]
        fn includes_increasing_straight() {
            assert_eq!(contains_increasing_straight(&"hijklmmn".to_string()), true);
        }

        #[test]
        fn does_not_include_increasing_straight() {
            assert_eq!(contains_increasing_straight(&"abbceffg".to_string()), false);
        }

        #[test]
        fn contains_bad_letters() {
            assert_eq!(find_bad_char(&"hijklmmn".to_string()), Some(1));
            assert_eq!(find_bad_char(&"hjoklmmn".to_string()), Some(2));
            assert_eq!(find_bad_char(&"hjjklmml".to_string()), Some(4));
        }

        #[test]
        fn does_not_contain_bad_letters() {
            assert_eq!(find_bad_char(&"abbceffg".to_string()), None);
        }

        #[test]
        fn includes_non_overlapping_pairs() {
            assert_eq!(contains_non_overlapping_pairs(&"abbceffg".to_string()), true);
            assert_eq!(contains_non_overlapping_pairs(&"abbbbabc".to_string()), true);
        }

        #[test]
        fn does_not_contain_non_overlapping_pairs() {
            assert_eq!(contains_non_overlapping_pairs(&"abbcegjk".to_string()), false);
            assert_eq!(contains_non_overlapping_pairs(&"abycegjk".to_string()), false);
        }
    }

    mod tests_increment_password {
        use super::*;

        #[test]
        fn test_increment_password_1() {
            assert_eq!(increment_chars("xx".to_string()), "xy");
        }

        #[test]
        fn test_increment_password_2() {
            assert_eq!(increment_chars("xz".to_string()), "ya");
        }

        #[test]
        fn test_increment_password_3() {
            assert_eq!(increment_chars("ya".to_string()), "yb");
        }

        #[test]
        fn test_increment_password_4() {
            assert_eq!(increment_chars("zz".to_string()), "ab");
        }
    }

    mod tests_next_valid_password {
        use super::*;

        #[test]
        fn test_get_next_password_1() {
            assert_eq!(next_valid_password("abcdefgh".to_string()), "abcdffaa");
        }

        #[test]
        fn test_get_next_password_2() {
            assert_eq!(next_valid_password("ghijklmn".to_string()), "ghjaabcc");
        }
    }
}
