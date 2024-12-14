use std::{collections::HashMap, ops::Range};

use chrono::{NaiveDateTime, Timelike};

use crate::solution::Solution;

pub struct Day04;

enum LogEvent {
    BeginShift { guard_id: usize },
    FallAsleep,
    WakeUp,
}

struct LogEntry {
    event: LogEvent,
    date_time: NaiveDateTime,
}

#[derive(Debug)]
struct Guard {
    id: usize,
    sleep_ranges: Vec<Range<u32>>,
}

struct MinuteFrequency {
    minute: usize,
    frequency: usize,
}

impl Guard {
    fn total_sleep_time(&self) -> u32 {
        self.sleep_ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum::<u32>()
    }

    fn most_frequent_minute(&self) -> MinuteFrequency {
        (0..60)
            .map(|minute| {
                let frequency = self
                    .sleep_ranges
                    .iter()
                    .filter(|range| range.contains(&(minute as u32)))
                    .count();
                MinuteFrequency { minute, frequency }
            })
            .max_by_key(|minute_frequency| minute_frequency.frequency)
            .expect("should contain at least one minute frequency entry")
    }
}

fn parse_logs(input: &str) -> Vec<LogEntry> {
    // Example log entries:
    // [1518-11-01 00:00] Guard #10 begins shift
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:25] wakes up

    let mut logs = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let tokens = line.split_once('[').unwrap().1.split_once(']').unwrap();
            let date_time = tokens.0.trim();
            let date_time = NaiveDateTime::parse_from_str(date_time, "%Y-%m-%d %H:%M")
                .expect("should be valid date time format");

            let event = tokens.1.trim();
            let event = if event.starts_with("Guard") {
                let guard_id = event
                    .split_once('#')
                    .unwrap()
                    .1
                    .split_once(' ')
                    .unwrap()
                    .0
                    .parse::<usize>()
                    .unwrap();
                LogEvent::BeginShift { guard_id }
            } else if event.starts_with("falls") {
                LogEvent::FallAsleep
            } else {
                LogEvent::WakeUp
            };
            Some(LogEntry { event, date_time })
        })
        .collect::<Vec<_>>();
    logs.sort_by_key(|log| log.date_time);
    logs
}

fn create_guards(logs: Vec<LogEntry>) -> Vec<Guard> {
    // iterate through logs and handle each event to find sleep windows for a guard
    let mut guards: HashMap<usize, Guard> = HashMap::new();
    let mut current_id = 0;
    let mut sleep_start = 0;
    for log in logs {
        match log.event {
            LogEvent::BeginShift { guard_id } => {
                current_id = guard_id;
            }
            LogEvent::FallAsleep => sleep_start = log.date_time.time().minute(),
            LogEvent::WakeUp => {
                let duration = Range {
                    start: sleep_start,
                    end: log.date_time.time().minute(),
                };
                guards
                    .entry(current_id)
                    .and_modify(|entry| {
                        entry.sleep_ranges.push(duration.clone());
                    })
                    .or_insert(Guard {
                        id: current_id,
                        sleep_ranges: vec![duration],
                    });
            }
        }
    }
    guards.into_values().collect::<Vec<Guard>>()
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        // Sort guards
        // Find guard will most sleep time
        // Given that guard, find the minute they slept the most
        let logs = parse_logs(input);
        let guards = create_guards(logs);

        // Find guard that sleeps the most
        let sleepiest_guard = guards
            .iter()
            .max_by_key(|guard| guard.total_sleep_time())
            .unwrap();
        let ans = sleepiest_guard.id * sleepiest_guard.most_frequent_minute().minute;
        format!("{ans}")
    }

    fn part2(&self, input: &str) -> String {
        let logs = parse_logs(input);
        let guards = create_guards(logs);
        let guard = guards
            .iter()
            .max_by_key(|guard| guard.most_frequent_minute().frequency)
            .unwrap();
        let ans = guard.id * guard.most_frequent_minute().minute;
        format!("{ans}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up

";

    #[test]
    fn part1_example() {
        assert_eq!(Day04.part1(INPUT), String::from("240"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(Day04.part2(INPUT), String::from("4455"));
    }
}
