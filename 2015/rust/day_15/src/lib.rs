use std::cmp::max;
use std::iter::zip;
use std::ops::Add;

pub fn process_part_1(input: &str) -> String {
    get_max_score(parse_input(input), None).to_string()
}

pub fn process_part_2(input: &str) -> String {
    get_max_score(parse_input(input), Some(500)).to_string()
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn apply_teaspoons(&self, teaspoons: i32) -> Ingredient {
        Ingredient {
            capacity: self.capacity * teaspoons,
            durability: self.durability * teaspoons,
            flavor: self.flavor * teaspoons,
            texture: self.texture * teaspoons,
            calories: self.calories * teaspoons,
        }
    }

    fn score(&self) -> i32 {
        if self.capacity < 0 || self.durability < 0 || self.flavor < 0 || self.texture < 0 {
            return 0;
        }
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}

impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input.lines()
        .map(|line| {
            let factors = line.replace(",", "")
                .split(" ")
                .filter_map(|token| {
                    token.parse::<i32>().ok()
                })
                .collect::<Vec<i32>>();
            Ingredient {
                capacity: factors[0],
                durability: factors[1],
                flavor: factors[2],
                texture: factors[3],
                calories: factors[4],
            }
        })
        .collect()
}

fn combine(ingredients: &Vec<Ingredient>, teaspoons: Vec<i32>) -> Ingredient {
    zip(ingredients, teaspoons)
        .fold(Ingredient::default(), |acc, (ing, n)| {
            acc + ing.apply_teaspoons(n)
        })
}

fn get_max_score_2(ingredients: Vec<Ingredient>, calorie_target: Option<i32>) -> i32 {
    let mut max_score = 0;
    for a in 1..97 {
        for b in 1..97 {
            if a + b != 100 {
                continue;
            }
            let combination = combine(&ingredients, vec![a, b]);
            if calorie_target.is_some() && calorie_target.unwrap() != combination.calories {
                continue;
            }
            max_score = max(max_score, combination.score())
        }
    }
    max_score
}

fn get_max_score_4(ingredients: Vec<Ingredient>, calorie_target: Option<i32>) -> i32 {
    let mut max_score = 0;
    for a in 1..97 {
        for b in 1..97 {
            for c in 1..97 {
                for d in 1..97 {
                    if a + b + c + d != 100 {
                        continue;
                    }
                    let combination = combine(&ingredients, vec![a, b, c, d]);
                    if calorie_target.is_some() && calorie_target.unwrap() != combination.calories {
                        continue;
                    }
                    max_score = max(max_score, combination.score())
                }
            }
        }
    }
    max_score
}

fn get_max_score(ingredients: Vec<Ingredient>, calorie_target: Option<i32>) -> i32 {
    if ingredients.len() == 2 {
        get_max_score_2(ingredients, calorie_target)
    } else {
        get_max_score_4(ingredients, calorie_target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

    #[test]
    fn part1() {
        assert_eq!(process_part_1(INPUT), "62842880");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part_2(INPUT), "57600000");
    }
}
