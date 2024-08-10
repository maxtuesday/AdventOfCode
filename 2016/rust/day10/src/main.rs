use std::collections::HashMap;

fn main() {
    let input = include_str!("../../../input/day10.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    parse(input);
    0
}

#[derive(Debug)]
struct Bot {
    low: u32,
    high: u32,
}

#[derive(Debug)]
struct BotConnection {
    id: String,
    low: String,
    high: String,
}

fn parse(input: &str) {
    let (bot_initial_values, bot_cmp_instructions): (Vec<_>, Vec<_>) =
        input.lines().partition(|line| line.starts_with("value"));

    // let mut bots: HashMap<String, Bot> = HashMap::new();
    // for line in bot_initial_values {
    //     let tokens = line.split(" ").collect::<Vec<_>>();
    //     let value = tokens.get(1).unwrap().parse::<u32>().unwrap();
    //     let bot = tokens[4..].join(" ");
    //     bots.entry(bot)
    //         .and_modify(|bot| {
    //             bot.low = bot.low.min(value);
    //             bot.high = bot.high.max(value);
    //         })
    //         .or_insert(Bot {
    //             low: value,
    //             high: value,
    //         });
    // }

    // println!("Init values:");
    // dbg!(&bots);

    // bot 88 gives low to bot 51 and high to bot 42
    // bot 70 gives low to output 10 and high to bot 4
    let connections = bot_cmp_instructions
        .into_iter()
        .map(|instruction| {
            let tokens = instruction.split(" ").collect::<Vec<_>>();
            let src_bot = tokens[0..2].join(" ");
            let low_dst = tokens[5..7].join(" ");
            let high_dst = tokens[10..].join(" ");
            (
                src_bot.clone(),
                BotConnection {
                    id: src_bot,
                    low: low_dst,
                    high: high_dst,
                },
            )
        })
        .collect::<HashMap<String, BotConnection>>();

    println!("Connections:");
    dbg!(&connections);

    let mut bot = bot_initial_values
        .iter()
        .find(|line| line.starts_with("value 61"));

    let path_61 = vec![];
    while let Some(bot) = bot {
        let key = bot.to_string();
        let connection = connections.get(&key).expect("there should be a bot connection");
        
    }

    let start_bot_17 = bot_initial_values
        .iter()
        .find(|line| line.starts_with("value 17"))
        .unwrap();


}

// fn process_instruction(instruction: &str)

// fn process_instructions(instructions: Vec<&str>, bots: &mut HashMap<String, Bot>) -> Result<(), Vec<&str>> {
//     let mut failed_instructions = vec![];

//     for instruction in instructions {
//         let tokens = instruction.split(" ").collect::<Vec<_>>();
//         let src_bot = tokens[0..2].join(" ");
//         let low_dst = tokens[5..7].join(" ");
//         let high_dst = tokens[10..].join(" ");

//         let src_bot = bots
//             .get(&src_bot)
//             .expect(format!("expected bot to exist...: {src_bot}").as_str());
//         let src_low = src_bot.low;
//         let src_high = src_bot.high;

//         bots.entry(low_dst)
//             .and_modify(|bot| {
//                 bot.low = bot.low.min(src_low);
//                 bot.high = bot.high.max(src_low);
//             })
//             .or_insert(Bot {
//                 low: src_low,
//                 high: src_low,
//             });

//         bots.entry(high_dst)
//             .and_modify(|bot| {
//                 bot.low = bot.low.min(src_high);
//                 bot.high = bot.high.max(src_high);
//             })
//             .or_insert(Bot {
//                 low: src_high,
//                 high: src_high,
//             });
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";
        assert_eq!(part1(input), 1);
    }
}
