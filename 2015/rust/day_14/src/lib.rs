struct Deer {
    deer_bio_info: DeerBioInfo,
    deer_state_info: DeerStateInfo,
}

struct DeerBioInfo {
    speed: i32,
    fly_duration: i32,
    rest_duration: i32,
}

enum DeerState {
    Move,
    Rest,
}

struct DeerStateInfo {
    state: DeerState,
    action_duration_left: i32,
    distance_traveled: i32,
    points: i32,
}

fn parse_input(input: &str) -> Vec<Deer> {
    input.lines().map(|line| {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let metrics = tokens.iter()
            .filter_map(|token| token.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Deer {
            deer_bio_info: DeerBioInfo {
                speed: metrics[0],
                fly_duration: metrics[1],
                rest_duration: metrics[2],
            },
            deer_state_info: DeerStateInfo {
                state: DeerState::Move,
                action_duration_left: metrics[1],
                distance_traveled: 0,
                points: 0,
            },
        }
    }).collect::<Vec<Deer>>()
}

fn simulate_second(deer: &mut Deer) {
    deer.deer_state_info.action_duration_left -= 1;
    match deer.deer_state_info.state {
        DeerState::Move => {
            deer.deer_state_info.distance_traveled += deer.deer_bio_info.speed;
            if deer.deer_state_info.action_duration_left == 0 {
                deer.deer_state_info.state = DeerState::Rest;
                deer.deer_state_info.action_duration_left = deer.deer_bio_info.rest_duration;
            }
        }
        DeerState::Rest => {
            if deer.deer_state_info.action_duration_left == 0 {
                deer.deer_state_info.state = DeerState::Move;
                deer.deer_state_info.action_duration_left = deer.deer_bio_info.fly_duration;
            }
        }
    }
}

fn simulate_deer(deer: &mut Vec<Deer>, duration: usize) {
    for _ in 0..=duration {
        for d in deer.iter_mut() {
            simulate_second(d);
        }
        let max_distance_deer = deer.iter_mut()
            .max_by(|a, b| {
                a.deer_state_info.distance_traveled.cmp(&b.deer_state_info.distance_traveled)
            })
            .unwrap();
        max_distance_deer.deer_state_info.points += 1;
    }
}

pub fn process_part_1(input: &str) -> String {
    let mut deer_squad = parse_input(input);
    simulate_deer(&mut deer_squad, 2503);
    let max_distance = deer_squad.iter()
        .max_by(|a, b| {
            a.deer_state_info.distance_traveled.cmp(&b.deer_state_info.distance_traveled)
        })
        .unwrap()
        .deer_state_info.distance_traveled;
    max_distance.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut deer_squad = parse_input(input);
    simulate_deer(&mut deer_squad, 2503);
    let max_score = deer_squad.iter()
        .max_by(|a, b| {
            a.deer_state_info.points.cmp(&b.deer_state_info.points)
        })
        .unwrap()
        .deer_state_info.points;
    max_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

    #[test]
    fn test_simulate_second() {
        let mut deer = parse_input(INPUT);
        simulate_second(&mut deer[0]);
        assert_eq!(deer[0].deer_state_info.distance_traveled, 14);
        simulate_second(&mut deer[1]);
        assert_eq!(deer[1].deer_state_info.distance_traveled, 16);
    }

    #[test]
    fn test_simulate_1000_seconds() {
        let mut deer = parse_input(INPUT);
        simulate_deer(&mut deer, 1000);
        assert_eq!(deer[0].deer_state_info.distance_traveled, 1120);
        assert_eq!(deer[1].deer_state_info.distance_traveled, 1056);
    }

    #[test]
    fn test_simulate_with_points_1000_seconds() {
        let mut deer = parse_input(INPUT);
        simulate_deer(&mut deer, 1000);
        assert_eq!(deer[0].deer_state_info.points, 312);
        assert_eq!(deer[1].deer_state_info.points, 689);
    }
}
