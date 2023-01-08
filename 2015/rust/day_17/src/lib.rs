pub fn process_part_1(input: &str) -> String {
    let containers = parse_input(input);
    let capacity = 150;
    let combinations = find_container_combinations(containers, capacity);
    combinations.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let containers = parse_input(input);
    let capacity = 150;
    let combinations = find_container_combinations(containers, capacity);
    let count = count_min_container_combinations(combinations);
    count.to_string()
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| {
            line.parse::<i32>().unwrap()
        })
        .collect::<Vec<_>>()
}

fn find_container_combinations(containers: Vec<i32>, capacity: i32) -> Vec<i32> {
    let mut res = vec![];
    dfs(0, containers, capacity, 0, &mut res);
    res
}

fn dfs(cur_capacity: i32,
       mut containers: Vec<i32>,
       capacity: i32,
       containers_used: i32,
       res: &mut Vec<i32>,
) {
    if cur_capacity == capacity {
        res.push(containers_used);
        return;
    }

    if containers.len() == 0 {
        return;
    }

    let top = containers.pop().unwrap();
    if cur_capacity + top <= capacity {
        // we can try and use this container
        dfs(cur_capacity + top,
            containers.clone(),
            capacity,
            containers_used + 1, res);
    }
    // skip container
    dfs(cur_capacity, containers, capacity, containers_used, res);
}

fn count_min_container_combinations(combinations: Vec<i32>) -> usize {
    let min_containers_used = combinations.iter().min().unwrap();
    combinations.iter()
        .filter(|count| *count == min_containers_used)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"20
15
10
5
5"#;

    #[test]
    fn test_fill_containers_combinations() {
        let containers = parse_input(INPUT);
        let capacity = 25;
        let combinations = find_container_combinations(containers, capacity);
        assert_eq!(combinations.len(), 4);
    }

    #[test]
    fn test_count_min_container_combinations() {
        let containers = parse_input(INPUT);
        let capacity = 25;
        let combinations = find_container_combinations(containers, capacity);
        let count = count_min_container_combinations(combinations);
        assert_eq!(count, 3);
    }
}
