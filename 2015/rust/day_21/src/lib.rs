const SHOP_WEAPONS: &str = r#"Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0"#;

const SHOP_ARMOR: &str = r#"None      0     0       0
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5"#;

const SHOP_RINGS: &str = r#"Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3"#;

#[derive(Debug)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

struct Shop {
    items: Vec<Item>,
}

struct Player {
    health: i32,
    damage: i32,
    armor: i32,
}

fn parse_shop(shop: &str) -> Shop {
    let items = shop.lines()
        .map(|line| {
            let mut tokens = line.split(" ").collect::<Vec<&str>>();
            let name = tokens[0].to_string();
            tokens.reverse();
            let nums = tokens.iter()
                .filter_map(|token| token.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            Item {
                name,
                cost: nums[2],
                damage: nums[1],
                armor: nums[0],
            }
        })
        .collect::<Vec<Item>>();
    Shop { items }
}

fn net_damage(damage: i32, armor: i32) -> i32 {
    let net_damage = damage - armor;
    net_damage.clamp(0, damage)
}

pub fn process_part_1(input: &str) -> String {
    let boss = Player {
        health: 100,
        damage: 8,
        armor: 2,
    };
    let weapons = parse_shop(SHOP_WEAPONS).items;
    let armor = parse_shop(SHOP_ARMOR).items;
    let rings = parse_shop(SHOP_RINGS).items;
    let no_ring = Item {
        name: "No Ring".to_string(),
        cost: 0,
        damage: 0,
        armor: 0,
    };
    let mut damage_rings = rings.iter()
        .filter(|ring| ring.damage > 0)
        .collect::<Vec<_>>();
    damage_rings.insert(0, &no_ring);
    let mut defence_rings = rings.iter()
        .filter(|ring| ring.armor > 0)
        .collect::<Vec<_>>();
    defence_rings.insert(0, &no_ring);

    // Buy:
    // 1 weapon
    // 0-1 armor
    // 0-2 rings

    let rounds_to_kill = weapons.iter()
        .map(|item| {
            let damage_done = net_damage(item.damage, boss.armor);
            let mut rounds = 0;
            let mut health = 100;
            while health > 0 {
                rounds += 1;
                health -= damage_done;
            }
            (item, rounds)
        })
        .collect::<Vec<_>>();
    dbg!(&rounds_to_kill);

    // how many turns can we survive with different armor ratings?
    let survival_rounds = armor.iter()
        .map(|item| {
            let damage_done = net_damage(boss.damage, item.armor);
            let mut rounds = 0;
            let mut health = 100;
            while health > 0 {
                rounds += 1;
                health -= damage_done;
            }
            (item, rounds)
        })
        .collect::<Vec<_>>();
    dbg!(&survival_rounds);

    todo!()
}

pub fn process_part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Hit Points: 100
Damage: 8
Armor: 2"#;

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "");
    }
}
