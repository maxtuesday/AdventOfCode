use serde_json::{Value};

pub fn process_part_1(input: &str) -> String {
    let json = parse_json(input);
    let res = sum_numbers(&json);
    res.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let json = parse_json(input);
    let res = sum_numbers_exclude_red(&json);
    res.to_string()
}

fn parse_json(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn sum_numbers(json: &Value) -> i64 {
    match json {
        Value::Number(num) => {
            num.as_i64().unwrap()
        }
        Value::Array(array) => {
            array.iter()
                .map(sum_numbers)
                .sum()
        }
        Value::Object(obj) => {
            obj.iter()
                .map(|(_, value) | sum_numbers(value))
                .sum()
        }
        _ => 0
    }
}

fn sum_numbers_exclude_red(json: &Value) -> i64 {
    match json {
        Value::Number(num) => {
            num.as_i64().unwrap()
        }
        Value::Array(array) => {
            array.iter()
                .map(sum_numbers_exclude_red)
                .sum()
        }
        Value::Object(obj) => {
            let contains_red = obj.values()
                .find(|value| value.as_str().unwrap_or("") == "red");
            if contains_red.is_none() {
                obj.iter()
                    .map(|(_, value)| sum_numbers_exclude_red(value))
                    .sum()
            } else {
                0
            }
        }
        _ => 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = r#"
            {
                "a": [1, 2, 3]
            }
        "#;
        assert_eq!(process_part_1(a), "6");
        let b = r#"
            {
                "a": {
                    "b":4,
                    "g":"yellow"
                },
                "c": -1
            }
        "#;
        assert_eq!(process_part_1(b), "3");
        let c = r#"
            {
                "a": [[[3]]]
            }
        "#;
        assert_eq!(process_part_1(c), "3");
    }

    #[test]
    fn part2() {
        let a = r#"
            {
                "a": [1, { "c": "red", "b": 2 } ,3]
            }
        "#;
        assert_eq!(process_part_2(a), "4");

        let b = r#"
            {
                "d": "red",
                "e": [1,2,3,4],
                "f": 5
            }
        "#;
        assert_eq!(process_part_2(b), "0");
    }
}
