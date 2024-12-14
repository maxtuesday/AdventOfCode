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
    logs.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    logs
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        // Sort guards
        // Find guard will most sleep time
        // Given that guard, find the minute they slept the most

        let logs = parse_logs(input);

        // iterate through logs and handle each event to find sleep windows for a guard

        let mut guards: HashMap<usize, Vec<Range<u32>>> = HashMap::new();

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
                            entry.push(duration.clone());
                        })
                        .or_insert(vec![duration]);
                }
            }
        }

        // Find guard that sleeps the most
        let mut guards_and_sleep_time = guards
            .iter()
            .map(|guard| {
                let total_sleep_time = guard
                    .1
                    .iter()
                    .map(|range| range.end - range.start)
                    .sum::<u32>();
                (guard, total_sleep_time)
            })
            .collect::<Vec<_>>();
        guards_and_sleep_time.sort_by(|a, b| b.1.cmp(&a.1));

        let sleepiest_guard = guards_and_sleep_time
            .first()
            .expect("should at least one guard");

        let mut minutes: Vec<usize> = vec![0; 60];
        (0..minutes.len()).for_each(|minute| {
            minutes[minute] = sleepiest_guard
                .0
                 .1
                .iter()
                .filter(|range| range.contains(&(minute as u32)))
                .count();
        });

        let mut most_slept_minute = 0;
        let mut max_freq = 0;
        minutes.into_iter().enumerate().for_each(|(minute, freq)| {
            if freq > max_freq {
                most_slept_minute = minute;
                max_freq = freq;
            }
        });

        let ans = sleepiest_guard.0 .0 * most_slept_minute;

        format!("{ans}")
    }

    fn part2(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
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
        assert_eq!(Day04 {}.part1(input), String::from("240"));
    }
}
