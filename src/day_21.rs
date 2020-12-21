use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_21_input.txt");

type FoodList<'a> = Vec<(HashSet<&'a str>, HashSet<&'a str>)>;

fn parse_input<'a>(input: &'a str) -> FoodList<'a> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.strip_suffix(")").unwrap().split("(contains");
            let (ingredients_str, allergens_str) = (parts.next().unwrap(), parts.next().unwrap());
            let ingredients = ingredients_str.split_whitespace().collect();
            let allergens = allergens_str.split(",").map(str::trim).collect();
            (ingredients, allergens)
        })
        .collect()
}

fn find_unsafe_ingredients<'a>(foods: &FoodList<'a>) -> HashSet<&'a str> {
    // Get the set of all allergens that exist across all foods
    let all_allergens: HashSet<&str> = foods
        .iter()
        .map(|(_i, a)| a.iter())
        .flatten()
        .map(|a| *a)
        .collect();

    // Populate initial set of possible allergens by ingredient; we'll narrow this down next
    let mut all_possible_allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (i_set, a_set) in foods.iter() {
        for ingredient in i_set.iter() {
            let possible_allergens = all_possible_allergens
                .entry(ingredient)
                .or_insert_with(HashSet::new);
            possible_allergens.extend(a_set.iter());
        }
    }

    let mut all_possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    for allergen in all_allergens.iter() {
        println!("Find ingredient for {}", allergen);
        let mut ingredient_set: HashSet<&str> = HashSet::new();
        for (i_set, a_set) in foods.iter() {
            if a_set.contains(allergen) {
                if ingredient_set.is_empty() {
                    ingredient_set.extend(i_set.clone());
                } else {
                    ingredient_set = ingredient_set.intersection(i_set).cloned().collect();
                }
                println!("\tPossible ingredients: {:?}", ingredient_set);
            }
        }
        all_possible_ingredients.insert(allergen, ingredient_set);
    }
    println!("\nPossible ingredients for each allergen:");
    for (k, v) in all_possible_ingredients.iter() {
        println!("{}: {:?}", k, v);
    }

    // Reduce allergens
    let mut known_allergens: HashMap<&str, &str> = HashMap::new();
    loop {
        let mut found_ingredient: Option<(&str, &str)> = None;
        for (a, i_set) in all_possible_ingredients.iter() {
            if i_set.len() == 1 {
                found_ingredient = Some((*i_set.iter().next().unwrap(), *a));
                break;
            }
        }
        if let Some(found) = found_ingredient {
            known_allergens.insert(found.0, found.1);
            for (_, i_set) in all_possible_ingredients.iter_mut() {
                i_set.remove(found.0);
            }
        } else {
            break;
        }
    }
    println!("\nKnown allergens by ingredient:");
    for (k, v) in known_allergens.iter() {
        println!("{}: {}", k, v);
    }
    assert!(
        all_possible_ingredients
            .iter()
            .all(|(_, ingredients)| ingredients.is_empty()),
        "Could not resolve all alergens"
    );

    known_allergens.keys().map(|i| *i).collect()
}

fn find_safe_ingredient_occurrences<'a>(
    foods: &FoodList<'a>,
    unsafe_ingredients: HashSet<&'a str>,
) -> usize {
    foods
        .iter()
        .map(|(i_set, _)| i_set)
        .flatten()
        .fold(0, |acc, i| {
            if !unsafe_ingredients.contains(i) {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn p1() -> usize {
    let food_list = parse_input(INPUT);
    let unsafe_ingredients = find_unsafe_ingredients(&food_list);
    find_safe_ingredient_occurrences(&food_list, unsafe_ingredients)
}

pub fn p2() -> usize {
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
        let food_list = parse_input(EXAMPLE);
        let unsafe_ingredients = find_unsafe_ingredients(&food_list);
        let result = find_safe_ingredient_occurrences(&food_list, unsafe_ingredients);
        assert_eq!(5, result);
    }

    #[test]
    fn p1_correct_answer() {
        let food_list = parse_input(INPUT);
        let unsafe_ingredients = find_unsafe_ingredients(&food_list);
        let result = find_safe_ingredient_occurrences(&food_list, unsafe_ingredients);
        assert_eq!(2798, result);
    }

    // #[test]
    // fn p2_simple() {}

    // #[test]
    // fn p2_example() {}
}
