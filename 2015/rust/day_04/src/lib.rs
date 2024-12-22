use crypto::digest::Digest;
use crypto::md5::Md5;

// Used this post for reference on how to speed up this code:
// https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7

pub fn process_part_1(input: &str) -> String {
    let key = input.as_bytes();
    let mut hasher = Md5::new();
    for i in 1..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output= [0;16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            return i.to_string()
        }
        hasher.reset();
    }
    panic!("Answer not found")
}

pub fn process_part_2(input: &str) -> String {
    let key = input.as_bytes();
    let mut hasher = Md5::new();
    for i in 1..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output= [0;16];
        hasher.result(&mut output);

        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_six == 0 {
            return i.to_string()
        }
        hasher.reset();
    }
    panic!("Answer not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(process_part_1("abcdef"), "609043");
    }

    #[test]
    fn part1_test2() {
        assert_eq!(process_part_1("pqrstuv"), "1048970");
    }
}
