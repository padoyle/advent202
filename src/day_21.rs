use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_21_input.txt");

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    let mut allergens_by_ingredient = HashMap::new();
    for line in input.lines() {
        let mut parts = line.strip_suffix(")").unwrap().split("(contains");
        let (ingredients, allergens) = (parts.next().unwrap(), parts.next().unwrap());
        let allergens: Vec<&str> = allergens
            .split(",")
            .map(|allergen| allergen.trim())
            .collect();
        for ingredient in ingredients.split_whitespace() {
            let possible_allergens = allergens_by_ingredient
                .entry(ingredient)
                .or_insert_with(HashMap::new);
            for allergen in allergens.iter() {
                let entry = possible_allergens.entry(*allergen).or_insert(0);
                *entry += 1;
            }
        }
    }

    for (k, v) in allergens_by_ingredient.iter() {
        println!("{}: {:?}", k, v);
    }
    allergens_by_ingredient
}

pub fn p1() -> u32 {
    0
}

pub fn p2() -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

    #[test]
    fn p1_example() {
        let allergens = parse_input(EXAMPLE);
        panic!("asdf")
    }

    // #[test]
    // fn p1_correct_answer() {}

    // #[test]
    // fn p2_simple() {}

    // #[test]
    // fn p2_example() {}
}
